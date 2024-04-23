use crate::DynamicMerkleTreeError;
use nacho_data_structures::{
    DoubleMerkleWitness, Field, FieldConversion, Sibling, SingleMerkleWitness, U256,
};
use nacho_macros::calculate_sibling_index;
use nacho_poseidon_hash::{create_poseidon_hasher, poseidon_hash, PoseidonHasher};
use std::{array, io::SeekFrom, path::Path};
use tokio::{
    fs::{create_dir_all, File, OpenOptions},
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
};

/// An alias to simplify `Result` monad.
type Result<T> = std::result::Result<T, DynamicMerkleTreeError>;

/// An on disk dynamic Merkle tree optimized for low disk usage and quick data access.
///
/// - The constant generic parameter `H` is the height of the Merkle tree.
/// - The constant generic parameter `L` is the sibling count of the Merkle tree which is always one less than the height.
///
/// # Examples
///
/// Create a new `DynamicMerkleTree` with 15 as the height:
///
/// ```rs
/// let dmt = DynamicMerkleTree::<15, 14>::new("/nacho/dynamic_merkle_tree").await?;
/// ```
///
/// Push a value to the first unused leaf:
///
/// ```rs
/// dmt.push_leaf(value).await?;
/// ```
///
/// Set the value of leaf 7:
///
/// ```rs
/// dmt.set_leaf(7, value).await?;
/// ```
///
/// Get the value of leaf 5:
///
/// ```rs
/// let value = dmt.get_leaf(5).await?;
/// ```
///
/// Get the witness of leaf 4:
///
/// ```rs
/// let witness = dmt.get_leaf(4).await?;
/// ```
///
/// Get the witness of leaf 6 and leaf 8:
///
/// ```rs
/// let witness = dmt.get_double_witness(6, 8).await?;
/// ```
///
/// Get the witness of the first unused leaf:
///
/// ```rs
/// let witness = dmt.get_unused_single_witness().await?;
/// ```
///
/// Get the root:
///
/// ```rs
/// let root = dmt.get_root().await?;
/// ```
///
pub struct DynamicMerkleTree<const H: usize, const L: usize> {
    /// The files that holds the Merkle tree data.
    files: [File; H],
    /// The field elements that represent the zero value and hashes of it for each height.
    zeroes: [Field; H],
    /// The hasher that is used to calculate Poseidon hashes of field elements.
    hasher: PoseidonHasher,
}

impl<const H: usize, const L: usize> DynamicMerkleTree<H, L> {
    /// The maximum number of leaves in the Merkle tree.
    const MAX_NUMBER_OF_LEAVES: u64 = u64::pow(2, H as u32 - 1);

    /// The maximum number of leaves in the Merkle tree.
    const MAX_INDEX: u64 = Self::MAX_NUMBER_OF_LEAVES - 1;

    /// Creates a new `DynamicMerkleTree` at the given path.
    ///
    /// - The constant generic parameter `H` is the height of the Merkle tree.
    /// - The constant generic parameter `L` is the sibling count of the Merkle tree which is always one less than the height.
    ///
    /// # Examples
    ///
    /// Create a new `DynamicMerkleTree` with 19 as the height:
    ///
    /// ```rs
    /// let dmt = DynamicMerkleTree::<19, 18>::new("/nacho/dynamic_merkle_tree").await?;
    /// ```
    ///
    pub async fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        create_dir_all(path).await?;

        let mut files = vec![];

        for h in 0..H {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path.join(h.to_string()))
                .await?;

            files.push(file)
        }

        let files = files
            .try_into()
            .map_err(|_| DynamicMerkleTreeError::Infallible)?;

        let zero: Field = 0.into();
        let mut zeroes = [zero; H];

        let mut hasher = create_poseidon_hasher();

        for i in 1..H {
            zeroes[i] = poseidon_hash(&mut hasher, &[zeroes[i - 1], zeroes[i - 1]]);
        }

        Ok(Self {
            files,
            zeroes,
            hasher,
        })
    }

    /// Returns the value at the given leaf of the Merkle tree.
    ///
    /// Returns zero for any unused leaf.
    ///
    /// # Examples
    ///
    /// Get the value of leaf 5:
    ///
    /// ```rs
    /// let value = dmt.get_leaf(5).await?;
    /// ```
    ///
    pub async fn get_leaf(&mut self, index: u64) -> Result<Field> {
        if index >= Self::MAX_NUMBER_OF_LEAVES {
            return Err(DynamicMerkleTreeError::IndexDoesntExist);
        }

        let leaves_file = &mut self.files[0];

        let leaves_file_len = leaves_file.metadata().await?.len();
        let padding = index * 32;
        let mut buf = [0; 32];

        if leaves_file_len <= padding {
            let u256 = U256(buf);
            return Ok(u256.to_fields()[0]);
        }

        leaves_file.seek(SeekFrom::Start(padding)).await?;
        leaves_file.read_exact(&mut buf).await?;

        let u256 = U256(buf);

        Ok(u256.to_fields()[0])
    }

    /// Sets the value at the given leaf of the Merkle tree.
    ///
    /// Returns an error if there is any unused index before the given leaf index.
    ///
    /// # Examples
    ///
    /// Set the value of leaf 4:
    ///
    /// ```rs
    /// dmt.set_leaf(4, value).await?;
    /// ```
    ///
    pub async fn set_leaf(&mut self, index: u64, value: Field) -> Result<()> {
        if index > Self::MAX_INDEX {
            return Err(DynamicMerkleTreeError::IndexDoesntExist);
        }

        let leaves_file_len = {
            let file = &mut self.files[0];
            file.metadata().await?.len()
        };

        let padding = index * 32;

        if padding > leaves_file_len {
            return Err(DynamicMerkleTreeError::UnusableIndex);
        }

        let mut current_value = value;
        let mut current_padding = padding;

        for j in 0..H - 1 {
            let current_file = &mut self.files[j];

            let parent_index = index / 2_u64.pow(j as u32 + 1);

            let sibling_index = {
                let t = index / 2_u64.pow(j as u32);
                if t % 2 == 0 {
                    t + 1
                } else {
                    t - 1
                }
            };

            let sibling_padding = sibling_index * 32;
            let sibling_is_left = sibling_index % 2 == 0;
            let file_len = current_file.metadata().await?.len();

            let sibling_value = if file_len == 0 || sibling_padding > file_len - 32 {
                self.zeroes[j]
            } else {
                let mut buf = [0_u8; 32];
                current_file.seek(SeekFrom::Start(sibling_padding)).await?;
                current_file.read_exact(&mut buf).await?;
                U256(buf).to_fields()[0]
            };

            let (left, right) = if sibling_is_left {
                (sibling_value, current_value)
            } else {
                (current_value, sibling_value)
            };

            let parent_value = poseidon_hash(&mut self.hasher, &[left, right]);

            current_file.seek(SeekFrom::Start(current_padding)).await?;
            current_file.write_all(&U256::from(current_value).0).await?;
            current_file.flush().await?;

            let next_file = &mut self.files[j + 1];

            let next_padding = parent_index * 32;

            next_file.seek(SeekFrom::Start(next_padding)).await?;
            next_file.write_all(&U256::from(parent_value).0).await?;
            next_file.flush().await?;

            current_value = parent_value;
            current_padding = next_padding;
        }

        Ok(())
    }

    /// Sets the value of the first unused leaf in the Merkle tree.
    ///
    /// # Examples
    ///
    /// Push a value to the first unused leaf:
    ///
    /// ```rs
    /// dmt.push_leaf(value).await?;
    /// ```
    ///
    pub async fn push_leaf(&mut self, value: Field) -> Result<u64> {
        let padding = {
            let file = &mut self.files[0];
            file.metadata().await?.len()
        };

        let index = padding / 32;

        self.set_leaf(index, value).await?;

        Ok(index)
    }

    /// Returns the Merkle witness of the given leaf of the Merkle tree.
    ///
    /// # Examples
    ///
    /// Get the witness of leaf 4:
    ///
    /// ```rs
    /// let witness = dmt.get_single_witness(4).await?;
    /// ```
    ///
    pub async fn get_single_witness(&mut self, index: u64) -> Result<SingleMerkleWitness<L>> {
        let mut siblings: [Sibling; L] = array::from_fn(|_| Sibling::default());

        let mut index = index;

        if index > Self::MAX_INDEX {
            return Err(DynamicMerkleTreeError::IndexDoesntExist);
        }

        for i in 0..L {
            let file = &mut self.files[i];
            let file_len = file.metadata().await?.len();

            let sibling_index = calculate_sibling_index!(index);

            let padding = sibling_index * 32;

            let sibling = if file_len == 0 || padding > file_len - 32 {
                self.zeroes[i].into()
            } else {
                let mut buf = [0_u8; 32];
                file.seek(SeekFrom::Start(padding)).await?;
                file.read_exact(&mut buf).await?;
                U256(buf)
            };

            siblings[i] = Sibling {
                value: sibling,
                is_left: sibling_index % 2 == 0,
            };

            index /= 2;
        }

        Ok(SingleMerkleWitness { siblings })
    }

    /// Returns the Merkle witness of the given leaves of the Merkle tree.
    ///
    /// # Examples
    ///
    /// Get the witness of leaf 6 and leaf 8:
    ///
    /// ```rs
    /// let witness = dmt.get_double_witness(6, 8).await?;
    /// ```
    ///
    pub async fn get_double_witness(
        &mut self,
        mut index_x1: u64,
        mut index_x2: u64,
    ) -> Result<DoubleMerkleWitness<L>> {
        let mut siblings_x1: [Sibling; L] = array::from_fn(|_| Sibling::default());
        let mut siblings_x2: [Sibling; L] = array::from_fn(|_| Sibling::default());
        let mut siblings_at: [bool; L] = [false; L];
        let mut siblings_found = false;

        if index_x1 > Self::MAX_INDEX || index_x2 > Self::MAX_INDEX {
            return Err(DynamicMerkleTreeError::IndexDoesntExist);
        }

        for i in 0..L {
            let file = &mut self.files[i];
            let file_len = file.metadata().await?.len();

            let sibling_index_x1 = calculate_sibling_index!(index_x1);
            let sibling_index_x2 = calculate_sibling_index!(index_x2);

            if sibling_index_x1 == index_x2 && !siblings_found {
                siblings_found = true;
                siblings_at[i] = true
            }

            let padding_x1 = sibling_index_x1 * 32;
            let padding_x2 = sibling_index_x2 * 32;

            let sibling_x1 = if file_len == 0 || padding_x1 > file_len - 32 {
                self.zeroes[i].into()
            } else {
                let mut buf = [0_u8; 32];
                file.seek(SeekFrom::Start(padding_x1)).await?;
                file.read_exact(&mut buf).await?;
                U256(buf)
            };

            let sibling_x2 = if file_len == 0 || padding_x2 > file_len - 32 {
                self.zeroes[i].into()
            } else {
                let mut buf = [0_u8; 32];
                file.seek(SeekFrom::Start(padding_x2)).await?;
                file.read_exact(&mut buf).await?;
                U256(buf)
            };

            siblings_x1[i] = Sibling {
                value: sibling_x1,
                is_left: sibling_index_x1 % 2 == 0,
            };
            siblings_x2[i] = Sibling {
                value: sibling_x2,
                is_left: sibling_index_x2 % 2 == 0,
            };

            index_x1 /= 2;
            index_x2 /= 2;
        }

        Ok(DoubleMerkleWitness {
            siblings_x1,
            siblings_x2,
            siblings_at,
        })
    }

    /// Returns the Merkle witness of the first unused leaf in the Merkle tree.
    ///
    /// # Examples
    ///
    /// Get the witness of the first unused leaf:
    ///
    /// ```rs
    /// let witness = dmt.get_unused_single_witness().await?;
    /// ```
    ///
    pub async fn get_unused_single_witness(&mut self) -> Result<SingleMerkleWitness<L>> {
        let leaves_file_len = self.files[0].metadata().await?.len();

        let index = leaves_file_len / 32;
        self.get_single_witness(index).await
    }

    /// Returns the root hash of the Merkle tree.
    ///
    /// # Examples
    ///
    /// Get the root:
    ///
    /// ```rs
    /// let root = dmt.get_root().await?;
    /// ```
    ///
    pub async fn get_root(&mut self) -> Result<Field> {
        let leaves_file_len = self.files[0].metadata().await?.len();
        let root_file = &mut self.files[H - 1];

        if leaves_file_len == 0 {
            return Ok(self.zeroes[H - 1]);
        }

        let mut buf = [0; 32];

        root_file.seek(SeekFrom::Start(0)).await?;
        root_file.read_exact(&mut buf).await?;

        let u256 = U256(buf);

        Ok(u256.to_fields()[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs::remove_dir_all;

    #[tokio::test]
    async fn creates_merkle_tree() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/dynamic_merkle_tree/creates_merkle_tree";

        let _ = DynamicMerkleTree::<2, 1>::new(dir).await?;

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn sets_and_gets_values() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/dynamic_merkle_tree/sets_and_gets_values";

        let mut dmt = DynamicMerkleTree::<5, 4>::new(dir).await?;

        assert_eq!(dmt.get_leaf(0).await?, Field::from(0));

        dmt.set_leaf(0, Field::from(42)).await?;

        assert_eq!(dmt.get_leaf(0).await?, Field::from(42));

        dmt.set_leaf(0, Field::from(7)).await?;

        assert_eq!(dmt.get_leaf(0).await?, Field::from(7));

        dmt.set_leaf(1, Field::from(5)).await?;

        assert_eq!(dmt.get_leaf(1).await?, Field::from(5));

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn pushes_values() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/dynamic_merkle_tree/pushes_values";
        let mut dmt = DynamicMerkleTree::<4, 3>::new(dir).await?;

        assert_eq!(dmt.get_leaf(0).await?, Field::from(0));

        dmt.push_leaf(Field::from(42)).await?;

        assert_eq!(dmt.get_leaf(0).await?, Field::from(42));

        dmt.push_leaf(Field::from(5)).await?;

        assert_eq!(dmt.get_leaf(1).await?, Field::from(5));

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn updates_root() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/dynamic_merkle_tree/updates_root";
        let mut dmt = DynamicMerkleTree::<6, 5>::new(dir).await?;

        assert_eq!(
            dmt.get_root().await?,
            "16556836945641263257329399459944072214107361158323688202689648863681494824075"
                .parse()
                .unwrap(),
        );

        dmt.set_leaf(0, Field::from(42)).await?;

        assert_eq!(
            dmt.get_root().await?,
            "11286019890017551530834192343235674860486670774006350997684329661758571079256"
                .parse()
                .unwrap(),
        );

        dmt.set_leaf(0, Field::from(41)).await?;

        assert_eq!(
            dmt.get_root().await?,
            "11232547598950213937608367704554422658642238970817973524109980433706405318743"
                .parse()
                .unwrap(),
        );

        dmt.set_leaf(1, Field::from(7)).await?;

        assert_eq!(
            dmt.get_root().await?,
            "14390539994532440242922216804268685968096299699558773027910939265009811713542"
                .parse()
                .unwrap(),
        );

        dmt.set_leaf(2, Field::from(19)).await?;

        assert_eq!(
            dmt.get_root().await?,
            "14405807291535085173183291373527489643529878893823965854142657852765785728874"
                .parse()
                .unwrap(),
        );

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn doesnt_set_unusable_index() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/dynamic_merkle_tree/doesnt_set_unusable_index";

        let mut dmt = DynamicMerkleTree::<42, 41>::new(dir).await?;

        let unusable_index = 1;

        let result = dmt.set_leaf(unusable_index, "42".parse().unwrap()).await;

        assert!(matches!(result, Err(DynamicMerkleTreeError::UnusableIndex)));

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn doesnt_set_non_existent_index() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let dir = "/tmp/nacho/tests/dynamic_merkle_tree/doesnt_set_non_existent_index";

        let mut dmt = DynamicMerkleTree::<42, 41>::new(dir).await?;

        let non_existent_index = 2_u64.pow(41);

        let result = dmt
            .set_leaf(non_existent_index, "42".parse().unwrap())
            .await;

        assert!(matches!(
            result,
            Err(DynamicMerkleTreeError::IndexDoesntExist)
        ));

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn gets_correct_single_witnesses() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let dir = "/tmp/nacho/tests/dynamic_merkle_tree/gets_correct_single_witnesses";
        let mut dmt = DynamicMerkleTree::<5, 4>::new(dir).await?;

        let single_witness = dmt.get_single_witness(0).await?;

        assert_eq!(
            single_witness
                .siblings
                .map(|sibling| sibling.value.to_fields()[0]),
            [
                "0".parse().unwrap(),
                "21565680844461314807147611702860246336805372493508489110556896454939225549736"
                    .parse()
                    .unwrap(),
                "2447983280988565496525732146838829227220882878955914181821218085513143393976"
                    .parse()
                    .unwrap(),
                "544619463418997333856881110951498501703454628897449993518845662251180546746"
                    .parse()
                    .unwrap(),
            ]
        );

        dmt.set_leaf(0, "42".parse().unwrap()).await?;

        let witness = dmt.get_single_witness(1).await?;

        assert_eq!(
            witness.siblings.map(|sibling| sibling.value.to_fields()[0]),
            [
                "42".parse().unwrap(),
                "21565680844461314807147611702860246336805372493508489110556896454939225549736"
                    .parse()
                    .unwrap(),
                "2447983280988565496525732146838829227220882878955914181821218085513143393976"
                    .parse()
                    .unwrap(),
                "544619463418997333856881110951498501703454628897449993518845662251180546746"
                    .parse()
                    .unwrap(),
            ]
        );

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn gets_correct_double_witnesses() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let dir = "/tmp/nacho/tests/dynamic_merkle_tree/gets_correct_double_witnesses";
        let mut dmt = DynamicMerkleTree::<5, 4>::new(dir).await?;

        let double_witness = dmt.get_double_witness(0, 1).await?;

        assert_eq!(
            double_witness
                .siblings_x1
                .map(|sibling| sibling.value.to_fields()[0]),
            [
                "0".parse().unwrap(),
                "21565680844461314807147611702860246336805372493508489110556896454939225549736"
                    .parse()
                    .unwrap(),
                "2447983280988565496525732146838829227220882878955914181821218085513143393976"
                    .parse()
                    .unwrap(),
                "544619463418997333856881110951498501703454628897449993518845662251180546746"
                    .parse()
                    .unwrap(),
            ]
        );

        assert_eq!(
            double_witness
                .siblings_x2
                .map(|sibling| sibling.value.to_fields()[0]),
            [
                "0".parse().unwrap(),
                "21565680844461314807147611702860246336805372493508489110556896454939225549736"
                    .parse()
                    .unwrap(),
                "2447983280988565496525732146838829227220882878955914181821218085513143393976"
                    .parse()
                    .unwrap(),
                "544619463418997333856881110951498501703454628897449993518845662251180546746"
                    .parse()
                    .unwrap(),
            ]
        );

        assert_eq!(double_witness.siblings_at, [true, false, false, false]);

        dmt.set_leaf(0, "42".parse().unwrap()).await?;

        let double_witness = dmt.get_double_witness(0, 1).await?;

        assert_eq!(
            double_witness
                .siblings_x1
                .map(|sibling| sibling.value.to_fields()[0]),
            [
                "0".parse().unwrap(),
                "21565680844461314807147611702860246336805372493508489110556896454939225549736"
                    .parse()
                    .unwrap(),
                "2447983280988565496525732146838829227220882878955914181821218085513143393976"
                    .parse()
                    .unwrap(),
                "544619463418997333856881110951498501703454628897449993518845662251180546746"
                    .parse()
                    .unwrap(),
            ]
        );

        assert_eq!(
            double_witness
                .siblings_x2
                .map(|sibling| sibling.value.to_fields()[0]),
            [
                "42".parse().unwrap(),
                "21565680844461314807147611702860246336805372493508489110556896454939225549736"
                    .parse()
                    .unwrap(),
                "2447983280988565496525732146838829227220882878955914181821218085513143393976"
                    .parse()
                    .unwrap(),
                "544619463418997333856881110951498501703454628897449993518845662251180546746"
                    .parse()
                    .unwrap(),
            ]
        );

        assert_eq!(double_witness.siblings_at, [true, false, false, false]);

        Ok(remove_dir_all(dir).await?)
    }
}
