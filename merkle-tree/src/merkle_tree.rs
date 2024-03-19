use crate::{MerkleTreeError, Witness};
use data_structures::{field_from_bytes, field_to_bytes, Field};
use poseidon_hash::{create_poseidon_hasher, poseidon_hash, PoseidonHasher};
use std::{io::SeekFrom, path::Path};
use tokio::{
    fs::{create_dir_all, File, OpenOptions},
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
};

type Result<T> = std::result::Result<T, MerkleTreeError>;

/// An on disk Merkle tree implementation optimized for low storage and constant memory usage.
///
/// The constant generic parameter `H` represents the height of the Merkle tree.
///
/// Data of each height is represented by its own dedicated file.
///
/// Hash of zeroes are calculated and cached during the initialization.
///
/// # Examples
///
/// Create an on disk Merkle tree at the given path with 42 as the height:
///
/// ```rs
/// let tree = MerkleTree::<42>::new("/tmp/nacho/tree").await?;
/// ```
///
/// Assign the value 42 to the leaf at the 7th index:
///
/// ```rs
/// tree.set(7, Field::from(42)).await?;
/// ```
///
/// Push the value 7 to the Merkle tree and get the index pushed to:
///
/// ```rs
/// let index = tree.push(7.into()).await?;
/// ```
///
/// Get the value of the leaf at the 7th index:
///
/// ```rs
/// let value = tree.get(7).await?;
/// ```
///
/// Get the root:
///
/// ```rs
/// let root = tree.root().await?;
/// ```
///
/// Get the witness of the leaf at the 7th index:
///
/// ```rs
/// let witness = tree.witness(7).await?;
/// ```
///
pub struct MerkleTree<const H: usize> {
    files: [File; H],
    zeroes: [Field; H],
    hasher: PoseidonHasher,
}

impl<const H: usize> MerkleTree<H> {
    /// The maximum number of leaves in the Merkle tree.
    const MAX_NUMBER_OF_LEAVES: u64 = u64::pow(2, H as u32 - 1);

    /// Creates a new `MerkleTree` at the given path.
    ///
    /// The constant generic parameter `H` is used to set the height of the Merkle tree.
    ///
    /// All data is stored in files inside the folder at the given path and they shouldn't be edited manually.
    ///
    /// # Examples
    ///
    /// Create a new Merkle tree with 42 as the height:
    ///
    /// ```rs
    /// let tree = MerkleTree::<42>::new().await?;
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

        let files = files.try_into().map_err(|_| MerkleTreeError::Infallible)?;

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

    /// Returns the root of the Merkle tree.
    ///
    /// # Examples
    ///
    /// Get the root:
    ///
    /// ```rs
    /// let root = tree.root().await?;
    /// ```
    ///
    pub async fn root(&mut self) -> Result<Field> {
        let leaves_file_len = self.files[0].metadata().await?.len();
        let root_file = &mut self.files[H - 1];

        if leaves_file_len == 0 {
            return Ok(self.zeroes[H - 1]);
        }

        let mut buf = [0; 32];

        root_file.seek(SeekFrom::Start(0)).await?;
        root_file.read_exact(&mut buf).await?;

        Ok(field_from_bytes(&buf))
    }

    /// Returns the value of the leaf at the given index of the Merkle tree.
    ///
    /// Returns zero for indexes that are not used yet.
    ///
    /// # Examples
    ///
    /// Get the value of the leaf at the 7th index:
    ///
    /// ```rs
    /// let value = tree.get(7).await?;
    /// ```
    ///
    pub async fn get(&mut self, index: u64) -> Result<Field> {
        if index >= Self::MAX_NUMBER_OF_LEAVES {
            return Err(MerkleTreeError::NonExistentIndex {
                given_index: index,
                highest_possible_index: Self::MAX_NUMBER_OF_LEAVES - 1,
            });
        }

        let leaves_file = &mut self.files[0];

        let leaves_file_len = leaves_file.metadata().await?.len();
        let padding = index * 32;
        let mut buf = [0; 32];

        if leaves_file_len <= padding {
            return Ok(field_from_bytes(&buf));
        }

        leaves_file.seek(SeekFrom::Start(padding)).await?;
        leaves_file.read_exact(&mut buf).await?;

        Ok(field_from_bytes(&buf))
    }

    /// Assigns the given value to the leaf at the given index of the Merkle tree.
    ///
    /// # Examples
    ///
    /// Assign the value 42 to the leaf at the 7th index:
    ///
    /// ```rs
    /// tree.set(7, Field::from(42)).await?;
    /// ```
    ///
    pub async fn set(&mut self, index: u64, value: Field) -> Result<()> {
        if index >= Self::MAX_NUMBER_OF_LEAVES {
            return Err(MerkleTreeError::NonExistentIndex {
                given_index: index,
                highest_possible_index: Self::MAX_NUMBER_OF_LEAVES - 1,
            });
        }

        let leaves_file_len = {
            let file = &mut self.files[0];
            file.metadata().await?.len()
        };

        let padding = index * 32;

        if padding > leaves_file_len {
            return Err(MerkleTreeError::UnusableIndex {
                given_index: index,
                usable_index: leaves_file_len / 32,
            });
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
                field_from_bytes(&buf)
            };

            let (left, right) = if sibling_is_left {
                (sibling_value, current_value)
            } else {
                (current_value, sibling_value)
            };

            let parent_value = poseidon_hash(&mut self.hasher, &[left, right]);

            current_file.seek(SeekFrom::Start(current_padding)).await?;
            current_file.write(&field_to_bytes(&current_value)).await?;
            current_file.flush().await?;

            let next_file = &mut self.files[j + 1];

            let next_padding = parent_index * 32;

            next_file.seek(SeekFrom::Start(next_padding)).await?;
            next_file.write(&field_to_bytes(&parent_value)).await?;
            next_file.flush().await?;

            current_value = parent_value;
            current_padding = next_padding;
        }

        Ok(())
    }

    /// Assigns the given value to the leaf at the latest unused index of the Merkle tree.
    ///
    /// # Examples
    ///
    /// Push the value 7 to the Merkle tree and get the index pushed to:
    ///
    /// ```rs
    /// let index = tree.push(7.into()).await?;
    /// ```
    ///
    pub async fn push(&mut self, value: Field) -> Result<u64> {
        let padding = {
            let file = &mut self.files[0];
            file.metadata().await?.len()
        };

        let index = padding / 32;

        self.set(index, value).await?;

        Ok(index)
    }

    /// Returns the witness of the leaf at the given index of the Merkle tree.
    ///
    /// # Examples
    ///
    /// Get the witness of the leaf at the 7th index:
    ///
    /// ```rs
    /// let witness = tree.witness(7).await?;
    /// ```
    ///
    pub async fn witness(&mut self, index: u64) -> Result<Witness<H>> {
        let mut witness = [self.zeroes[0]; H];

        if index >= Self::MAX_NUMBER_OF_LEAVES {
            return Err(MerkleTreeError::NonExistentIndex {
                given_index: index,
                highest_possible_index: Self::MAX_NUMBER_OF_LEAVES - 1,
            });
        }

        witness[0] = Field::from(index);

        for i in 0..H - 1 {
            let h = i as u32;

            let file = &mut self.files[i];

            let file_len = file.metadata().await?.len();

            let sibling_index = {
                let t = index / 2_u64.pow(h);
                if t % 2 == 0 {
                    t + 1
                } else {
                    t - 1
                }
            };

            let sibling_padding = sibling_index * 32;

            let sibling = if file_len == 0 || sibling_padding > file_len - 32 {
                self.zeroes[h as usize]
            } else {
                let mut buf = [0_u8; 32];
                file.seek(SeekFrom::Start(sibling_padding)).await?;
                file.read_exact(&mut buf).await?;
                field_from_bytes(&buf)
            };

            witness[h as usize + 1] = sibling;
        }
        Ok(Witness::new(witness))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs::remove_dir_all;

    #[tokio::test]
    async fn creates_merkle_tree() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/merkle_tree/creates_merkle_tree";

        let _ = MerkleTree::<2>::new(dir).await?;

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn sets_and_gets_values() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/merkle_tree/sets_and_gets_values";

        let mut tree = MerkleTree::<5>::new(dir).await?;

        assert_eq!(tree.get(0).await?, Field::from(0));

        tree.set(0, Field::from(42)).await?;

        assert_eq!(tree.get(0).await?, Field::from(42));

        tree.set(0, Field::from(7)).await?;

        assert_eq!(tree.get(0).await?, Field::from(7));

        tree.set(1, Field::from(5)).await?;

        assert_eq!(tree.get(1).await?, Field::from(5));

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn pushes_values() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/merkle_tree/pushes_values";
        let mut tree = MerkleTree::<4>::new(dir).await?;

        assert_eq!(tree.get(0).await?, Field::from(0));

        tree.push(Field::from(42)).await?;

        assert_eq!(tree.get(0).await?, Field::from(42));

        tree.push(Field::from(5)).await?;

        assert_eq!(tree.get(1).await?, Field::from(5));

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn updates_root() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/merkle_tree/updates_root";
        let mut tree = MerkleTree::<6>::new(dir).await?;

        assert_eq!(
            tree.root().await?,
            "16556836945641263257329399459944072214107361158323688202689648863681494824075"
                .parse()
                .unwrap(),
        );

        tree.set(0, Field::from(42)).await?;

        assert_eq!(
            tree.root().await?,
            "11286019890017551530834192343235674860486670774006350997684329661758571079256"
                .parse()
                .unwrap(),
        );

        tree.set(0, Field::from(41)).await?;

        assert_eq!(
            tree.root().await?,
            "11232547598950213937608367704554422658642238970817973524109980433706405318743"
                .parse()
                .unwrap(),
        );

        tree.set(1, Field::from(7)).await?;

        assert_eq!(
            tree.root().await?,
            "14390539994532440242922216804268685968096299699558773027910939265009811713542"
                .parse()
                .unwrap(),
        );

        tree.set(2, Field::from(19)).await?;

        assert_eq!(
            tree.root().await?,
            "14405807291535085173183291373527489643529878893823965854142657852765785728874"
                .parse()
                .unwrap(),
        );

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn doesnt_set_unusable_index() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/merkle_tree/doesnt_set_unusable_index";

        let mut tree = MerkleTree::<42>::new(dir).await?;

        let unusable_index = 1;

        match tree
            .set(unusable_index, "42".parse().unwrap())
            .await
            .unwrap_err()
        {
            MerkleTreeError::UnusableIndex {
                given_index,
                usable_index,
            } => {
                assert_eq!(given_index, unusable_index);
                assert_eq!(usable_index, 0);
            }
            _ => unreachable!(),
        }

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn doesnt_set_non_existent_index() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let dir = "/tmp/nacho/tests/merkle_tree/doesnt_set_non_existent_index";

        let mut tree = MerkleTree::<42>::new(dir).await?;

        let non_existent_index = 2_u64.pow(41);

        match tree
            .set(non_existent_index, "42".parse().unwrap())
            .await
            .unwrap_err()
        {
            MerkleTreeError::NonExistentIndex {
                given_index,
                highest_possible_index,
            } => {
                assert_eq!(given_index, non_existent_index);
                assert_eq!(highest_possible_index, 2_u64.pow(41) - 1);
            }
            _ => unreachable!(),
        }

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn gets_witness() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/merkle_tree/gets_witness";
        let mut tree = MerkleTree::<5>::new(dir).await?;

        let witness = tree.witness(0).await?;

        assert_eq!(
            witness.0,
            [
                "0".parse().unwrap(),
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

        tree.set(0, "42".parse().unwrap()).await?;

        let witness = tree.witness(1).await?;

        assert_eq!(
            witness.0,
            [
                "1".parse().unwrap(),
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
}
