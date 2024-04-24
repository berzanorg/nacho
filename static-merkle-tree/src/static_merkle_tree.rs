use crate::StaticMerkleTreeError;
use nacho_data_structures::{
    ByteConversion, Field, FieldConversion, Sibling, SingleMerkleWitness, U256,
};
use nacho_poseidon_hash::{create_poseidon_hasher, poseidon_hash, PoseidonHasher};
use std::{array, io::SeekFrom, path::Path};
use tokio::{
    fs::{create_dir_all, File, OpenOptions},
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
};

/// An alias to simplify `Result` monad.
type Result<T> = std::result::Result<T, StaticMerkleTreeError>;

/// An on disk static Merkle tree optimized for efficiently reading and modifying the tree.
///
/// - The constant generic parameter `H` is the height of the Merkle tree.
/// - The constant generic parameter `L` is the sibling count of the Merkle tree which is always one less than the height.
///
/// # Examples
///
/// Create a new `StaticMerkleTree` with 15 as the height:
///
/// ```rs
/// let smt = StaticMerkleTree::<15, 14>::new("/nacho/static_merkle_tree").await?;
/// ```
///
/// Set the value of leaf 7:
///
/// ```rs
/// smt.set_leaf(7, value).await?;
/// ```
///
/// Get the value of leaf 5:
///
/// ```rs
/// let value = smt.get_leaf(5).await?;
/// ```
///
/// Get the witness of leaf 4:
///
/// ```rs
/// let witness = smt.get_leaf(4).await?;
/// ```
///
/// Get the root:
///
/// ```rs
/// let root = smt.get_root().await?;
/// ```
///
pub struct StaticMerkleTree<const H: usize, const L: usize> {
    /// The file that holds the Merkle tree data.
    file: File,
    /// The hasher that is used to calculate Poseidon hashes of field elements.
    hasher: PoseidonHasher,
}

impl<const H: usize, const L: usize> StaticMerkleTree<H, L> {
    const TREE_SIZE_IN_BYTES: u64 = 32 * (2_u64.pow(H as u32) - 1);
    const MAX_LEAF_INDEX: u64 = 2_u64.pow(H as u32 - 1) - 1;
    const ROOT_LEAF_PADDING_IN_BYTES: u64 = 32 * (2_u64.pow(H as u32) - 2);

    /// Creates a new `StaticMerkleTree` at the given path.
    ///
    /// - The constant generic parameter `H` is the height of the Merkle tree.
    /// - The constant generic parameter `L` is the sibling count of the Merkle tree which is always one less than the height.
    ///
    /// # Examples
    ///
    /// Create a new `StaticMerkleTree` with 19 as the height:
    ///
    /// ```rs
    /// let smt = StaticMerkleTree::<19, 18>::new("/nacho/static_merkle_tree").await?;
    /// ```
    ///
    pub async fn new(path: impl AsRef<Path>) -> Result<StaticMerkleTree<H, L>> {
        let path = path.as_ref();

        let parent_dir_path = path
            .parent()
            .ok_or(StaticMerkleTreeError::ParentDirectoryNotSpecified)?
            .to_string_lossy()
            .to_string();

        if parent_dir_path.is_empty() {
            return Err(StaticMerkleTreeError::ParentDirectoryNotSpecified);
        }

        create_dir_all(parent_dir_path).await?;

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .await?;

        let mut hasher = create_poseidon_hasher();

        if file.metadata().await?.len() != Self::TREE_SIZE_IN_BYTES {
            let mut current_zero: Field = 0.into();
            let mut padding = 0;

            for i in 0..H {
                let leaves_count = 2_usize.pow(H as u32 - i as u32 - 1);

                let bytes_repr = U256::from(current_zero).to_bytes();

                let buf: Vec<_> = vec![bytes_repr; leaves_count]
                    .into_iter()
                    .flatten()
                    .collect();

                file.seek(SeekFrom::Start(padding)).await?;
                file.write_all(&buf).await?;
                file.flush().await?;

                current_zero = poseidon_hash(&mut hasher, &[current_zero, current_zero]);
                padding += 32 * leaves_count as u64;
            }
        }

        Ok(Self { file, hasher })
    }

    /// Sets the value at the given leaf of the Merkle tree.
    ///
    /// # Examples
    ///
    /// Set the value of leaf 4:
    ///
    /// ```rs
    /// smt.set_leaf(4, value).await?;
    /// ```
    ///
    pub async fn set_leaf(&mut self, leaf_index: u64, value: Field) -> Result<()> {
        if leaf_index > Self::MAX_LEAF_INDEX {
            return Err(StaticMerkleTreeError::LeafIndexesExceeded);
        }

        let mut cumulative_padding = 0;
        let mut current_leaf_index = leaf_index;
        let mut current_value = value;

        for i in 0..H {
            let leaves_count = 2_usize.pow(H as u32 - i as u32 - 1);
            let current_padding = 32 * current_leaf_index;

            let buf_repr = U256::from(current_value).to_bytes();

            self.file
                .seek(SeekFrom::Start(cumulative_padding + current_padding))
                .await?;
            self.file.write_all(&buf_repr).await?;
            self.file.flush().await?;

            if i == H - 1 {
                break;
            }

            let sibling_is_left = current_leaf_index % 2 == 1;

            let sibling_leaf_index = if sibling_is_left {
                current_leaf_index - 1
            } else {
                current_leaf_index + 1
            };

            let sibling_padding = 32 * sibling_leaf_index;

            let mut sibling_buf = [0u8; 32];

            self.file
                .seek(SeekFrom::Start(cumulative_padding + sibling_padding))
                .await?;
            self.file.read_exact(&mut sibling_buf).await?;

            let sibling_value: Field = U256::from_bytes(&sibling_buf).to_fields()[0];

            let (left_value, right_value) = if sibling_is_left {
                (sibling_value, current_value)
            } else {
                (current_value, sibling_value)
            };

            cumulative_padding += 32 * leaves_count as u64;
            current_leaf_index = current_leaf_index / 2;
            current_value = poseidon_hash(&mut self.hasher, &[left_value, right_value]);
        }

        Ok(())
    }

    /// Returns the value at the given leaf of the Merkle tree.
    ///
    /// # Examples
    ///
    /// Get the value of leaf 5:
    ///
    /// ```rs
    /// let value = smt.get_leaf(5).await?;
    /// ```
    ///
    pub async fn get_leaf(&mut self, leaf_index: u64) -> Result<Field> {
        if leaf_index > Self::MAX_LEAF_INDEX {
            return Err(StaticMerkleTreeError::LeafIndexesExceeded);
        }

        let padding = 32 * leaf_index;

        let mut buf = [0u8; 32];

        self.file.seek(SeekFrom::Start(padding)).await?;
        self.file.read_exact(&mut buf).await?;

        let value = U256(buf).to_fields()[0];

        Ok(value)
    }

    /// Returns the Merkle witness of the given leaf of the Merkle tree.
    ///
    /// # Examples
    ///
    /// Get the witness of leaf 4:
    ///
    /// ```rs
    /// let witness = smt.get_single_witness(4).await?;
    /// ```
    ///
    pub async fn get_single_witness(&mut self, leaf_index: u64) -> Result<SingleMerkleWitness<L>> {
        if leaf_index > Self::MAX_LEAF_INDEX {
            return Err(StaticMerkleTreeError::LeafIndexesExceeded);
        }

        let mut siblings = array::from_fn(|_| Sibling::default());

        let mut current_leaf_index = leaf_index;
        let mut cumulative_padding = 0;

        for i in 0..H - 1 {
            let leaves_count = 2_usize.pow(H as u32 - i as u32 - 1);

            let sibling_is_left = current_leaf_index % 2 == 1;

            let sibling_leaf_index = if sibling_is_left {
                current_leaf_index - 1
            } else {
                current_leaf_index + 1
            };

            let sibling_padding = 32 * sibling_leaf_index;

            let mut sibling_buf = [0u8; 32];

            self.file
                .seek(SeekFrom::Start(cumulative_padding + sibling_padding))
                .await?;
            self.file.read_exact(&mut sibling_buf).await?;

            let sibling_value = U256::from_bytes(&sibling_buf);

            siblings[i] = Sibling {
                value: sibling_value,
                is_left: sibling_is_left,
            };

            current_leaf_index = current_leaf_index / 2;
            cumulative_padding += 32 * leaves_count as u64;
        }

        Ok(SingleMerkleWitness { siblings })
    }

    /// Returns the root hash of the Merkle tree.
    ///
    /// # Examples
    ///
    /// Get the root hash:
    ///
    /// ```rs
    /// let root = smt.get_root().await?;
    /// ```
    ///
    pub async fn get_root(&mut self) -> Result<Field> {
        let padding = Self::ROOT_LEAF_PADDING_IN_BYTES;

        let mut buf = [0u8; 32];

        self.file.seek(SeekFrom::Start(padding)).await?;
        self.file.read_exact(&mut buf).await?;

        let value = U256(buf).to_fields()[0];

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs::remove_file;

    #[tokio::test]
    async fn creates_merkle_tree() {
        let dir = "/tmp/nacho/tests/static_merkle_tree/creates_merkle_tree";

        let smt = StaticMerkleTree::<2, 1>::new(dir).await.unwrap();

        let _ = smt;

        remove_file(dir).await.unwrap()
    }

    #[tokio::test]
    async fn sets_and_gets_leaves() {
        let dir = "/tmp/nacho/tests/static_merkle_tree/sets_and_gets_leaves";

        let mut smt = StaticMerkleTree::<18, 17>::new(dir).await.unwrap();

        smt.set_leaf(0, 12.into()).await.unwrap();
        smt.set_leaf(45, 7.into()).await.unwrap();
        smt.set_leaf(156, 267.into()).await.unwrap();

        assert_eq!(smt.get_leaf(0).await.unwrap(), 12.into());
        assert_eq!(smt.get_leaf(1).await.unwrap(), 0.into());
        assert_eq!(smt.get_leaf(45).await.unwrap(), 7.into());
        assert_eq!(smt.get_leaf(156).await.unwrap(), 267.into());
        assert_eq!(smt.get_leaf(99999).await.unwrap(), 0.into());

        remove_file(dir).await.unwrap()
    }

    #[tokio::test]
    async fn updates_root() {
        let dir = "/tmp/nacho/tests/static_merkle_tree/updates_root";

        let mut smt = StaticMerkleTree::<19, 18>::new(dir).await.unwrap();

        let root = smt.get_root().await.unwrap();

        assert_eq!(
            root,
            "25436453236035485996795240493313170211557120058262356001829805101279552630634"
                .parse()
                .unwrap()
        );

        smt.set_leaf(0, 435.into()).await.unwrap();

        let root = smt.get_root().await.unwrap();

        assert_eq!(
            root,
            "9483955031203401160713579395725671566345301961932530213516288103888164633380"
                .parse()
                .unwrap()
        );

        smt.set_leaf(85, 685.into()).await.unwrap();

        let root = smt.get_root().await.unwrap();

        assert_eq!(
            root,
            "17336108694741431772855439559487367699612615888710642999387305390429680201132"
                .parse()
                .unwrap()
        );

        remove_file(dir).await.unwrap()
    }

    #[tokio::test]
    async fn gets_witnesses() {
        let dir = "/tmp/nacho/tests/static_merkle_tree/gets_witnesses";

        let mut smt = StaticMerkleTree::<19, 18>::new(dir).await.unwrap();

        let witness = smt.get_single_witness(0).await.unwrap();

        assert_eq!(
            witness
                .siblings
                .clone()
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
                "20468198949394563802460512965219839480612000520504690501918527632215047268421"
                    .parse()
                    .unwrap(),
                "16556836945641263257329399459944072214107361158323688202689648863681494824075"
                    .parse()
                    .unwrap(),
                "15433636137932294330522564897643259724602670702144398296133714241278885195605"
                    .parse()
                    .unwrap(),
                "14472842460125086645444909368571209079194991627904749620726822601198914470820"
                    .parse()
                    .unwrap(),
                "21614416876217972474084851109688329000791937035724439964738173004620435920527"
                    .parse()
                    .unwrap(),
                "23396673455667782815008357063662227432928854130481827049177088579579506912772"
                    .parse()
                    .unwrap(),
                "16799216270319797546551726730220821530700130944535729528697378284641302758053"
                    .parse()
                    .unwrap(),
                "13496994890596928254174830027007320908142597452643688487140586680795895052589"
                    .parse()
                    .unwrap(),
                "3136367688481366987314253891173247447839122679172869317967104414474412425595"
                    .parse()
                    .unwrap(),
                "16414894720763442886261603851925762864778244212151669304308726942427436045416"
                    .parse()
                    .unwrap(),
                "22589430138891598861557640031858114956845676321373998952831060515802332123931"
                    .parse()
                    .unwrap(),
                "5791459643284782105200605043590392479676954402958680213128780189628932971164"
                    .parse()
                    .unwrap(),
                "16510281280427274356185141347114409143460588060107208117935011691496157196057"
                    .parse()
                    .unwrap(),
                "14486316384664676462434146886055824678546254125476991527791532204933765704550"
                    .parse()
                    .unwrap()
            ]
        );

        assert_eq!(
            witness.siblings.map(|sibling| sibling.is_left),
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false
            ]
        );

        smt.set_leaf(0, 435.into()).await.unwrap();

        let witness = smt.get_single_witness(0).await.unwrap();

        assert_eq!(
            witness
                .siblings
                .clone()
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
                "20468198949394563802460512965219839480612000520504690501918527632215047268421"
                    .parse()
                    .unwrap(),
                "16556836945641263257329399459944072214107361158323688202689648863681494824075"
                    .parse()
                    .unwrap(),
                "15433636137932294330522564897643259724602670702144398296133714241278885195605"
                    .parse()
                    .unwrap(),
                "14472842460125086645444909368571209079194991627904749620726822601198914470820"
                    .parse()
                    .unwrap(),
                "21614416876217972474084851109688329000791937035724439964738173004620435920527"
                    .parse()
                    .unwrap(),
                "23396673455667782815008357063662227432928854130481827049177088579579506912772"
                    .parse()
                    .unwrap(),
                "16799216270319797546551726730220821530700130944535729528697378284641302758053"
                    .parse()
                    .unwrap(),
                "13496994890596928254174830027007320908142597452643688487140586680795895052589"
                    .parse()
                    .unwrap(),
                "3136367688481366987314253891173247447839122679172869317967104414474412425595"
                    .parse()
                    .unwrap(),
                "16414894720763442886261603851925762864778244212151669304308726942427436045416"
                    .parse()
                    .unwrap(),
                "22589430138891598861557640031858114956845676321373998952831060515802332123931"
                    .parse()
                    .unwrap(),
                "5791459643284782105200605043590392479676954402958680213128780189628932971164"
                    .parse()
                    .unwrap(),
                "16510281280427274356185141347114409143460588060107208117935011691496157196057"
                    .parse()
                    .unwrap(),
                "14486316384664676462434146886055824678546254125476991527791532204933765704550"
                    .parse()
                    .unwrap()
            ]
        );

        assert_eq!(
            witness.siblings.map(|sibling| sibling.is_left),
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false
            ]
        );

        smt.set_leaf(85, 685.into()).await.unwrap();

        let witness = smt.get_single_witness(42).await.unwrap();

        assert_eq!(
            witness
                .siblings
                .clone()
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
                "20468198949394563802460512965219839480612000520504690501918527632215047268421"
                    .parse()
                    .unwrap(),
                "4634928718464554397739609044028692299605741010323962508454757219290440362024"
                    .parse()
                    .unwrap(),
                "1250935003133161145562566742231784049374770790421194102685733568202465496596"
                    .parse()
                    .unwrap(),
                "14472842460125086645444909368571209079194991627904749620726822601198914470820"
                    .parse()
                    .unwrap(),
                "21614416876217972474084851109688329000791937035724439964738173004620435920527"
                    .parse()
                    .unwrap(),
                "23396673455667782815008357063662227432928854130481827049177088579579506912772"
                    .parse()
                    .unwrap(),
                "16799216270319797546551726730220821530700130944535729528697378284641302758053"
                    .parse()
                    .unwrap(),
                "13496994890596928254174830027007320908142597452643688487140586680795895052589"
                    .parse()
                    .unwrap(),
                "3136367688481366987314253891173247447839122679172869317967104414474412425595"
                    .parse()
                    .unwrap(),
                "16414894720763442886261603851925762864778244212151669304308726942427436045416"
                    .parse()
                    .unwrap(),
                "22589430138891598861557640031858114956845676321373998952831060515802332123931"
                    .parse()
                    .unwrap(),
                "5791459643284782105200605043590392479676954402958680213128780189628932971164"
                    .parse()
                    .unwrap(),
                "16510281280427274356185141347114409143460588060107208117935011691496157196057"
                    .parse()
                    .unwrap(),
                "14486316384664676462434146886055824678546254125476991527791532204933765704550"
                    .parse()
                    .unwrap()
            ]
        );

        assert_eq!(
            witness.siblings.map(|sibling| sibling.is_left),
            [
                false, true, false, true, false, true, false, false, false, false, false, false,
                false, false, false, false, false, false
            ]
        );

        remove_file(dir).await.unwrap()
    }

    #[tokio::test]
    async fn calculates_correct_roots() {
        let dir = "/tmp/nacho/tests/static_merkle_tree/calculates_correct_roots";

        let mut smt = StaticMerkleTree::<19, 18>::new(dir).await.unwrap();

        let root = smt.get_root().await.unwrap();

        assert_eq!(
            root,
            "25436453236035485996795240493313170211557120058262356001829805101279552630634"
                .parse()
                .unwrap()
        );

        smt.set_leaf(0, 12.into()).await.unwrap();

        let root = smt.get_root().await.unwrap();

        assert_eq!(
            root,
            "16923653477795052403550863404160331876415078567876437852226001093716430402677"
                .parse()
                .unwrap()
        );

        remove_file(dir).await.unwrap();
    }
}
