use crate::{
    SingleWithdrawalWitness, WithdrawalsDbError, WITHDRAWALS_TREE_HEIGHT,
    WITHDRAWALS_TREE_SIBLING_COUNT, WITHDRAWAL_SIZE_IN_BYTES,
};
use nacho_data_structures::{ByteConversion, Field, FieldConversion, Withdrawal};
use nacho_dynamic_list::DynamicList;
use nacho_poseidon_hash::{create_poseidon_hasher, poseidon_hash, PoseidonHasher};
use nacho_static_merkle_tree::StaticMerkleTree;
use std::path::Path;

/// An alias to simplify `Result` monad.
type Result<T> = std::result::Result<T, WithdrawalsDbError>;

/// The database that stores withdrawals made from layer 2 network to layer 1 network.
///
/// # Examples
///
/// Create a new `WithdrawalsDb`:
///
/// ```rs
/// let withdrawals_db = WithdrawalsDb::new("/nacho/withdrawals_db").await?;
/// ```
///
/// Set the given withdrawal at the index 4:
///
/// ```rs
/// withdrawals_db.set(4, &withdrawal).await?;
/// ```
///
/// Get the withdrawal at the index 2:
///
/// ```rs
/// let withdrawal = withdrawals_db.get(2).await?;
/// ```
///
/// Get the witness of the withdrawal at the index 3:
///
/// ```rs
/// let witness = withdrawals_db.get_witness(3).await?;
/// ```
///
/// Get the root:
///
/// ```rs
/// let root = withdrawals_db.get_root().await?;
/// ```
///
pub struct WithdrawalsDb {
    /// The dynamic list that holds `Withdrawal` items on disk.
    list: DynamicList<WITHDRAWAL_SIZE_IN_BYTES>,
    /// The static Merkle tree that holds the hashes of `Withdrawal` items on disk.
    tree: StaticMerkleTree<WITHDRAWALS_TREE_HEIGHT, WITHDRAWALS_TREE_SIBLING_COUNT>,
    /// The hasher that is used to calculate Poseidon hashes of field elements.
    hasher: PoseidonHasher,
}

impl WithdrawalsDb {
    /// Creates a new `WithdrawalsDb` at the given path.
    ///
    /// # Examples
    ///
    /// Create a new `WithdrawalsDb`:
    ///
    /// ```rs
    /// let withdrawals_db = WithdrawalsDb::new("/nacho/withdrawals_db").await?;
    /// ```
    ///
    pub async fn new(path: impl AsRef<Path>) -> Result<WithdrawalsDb> {
        let path = path.as_ref();

        let list = DynamicList::new(path.join("dynamic_list")).await?;
        let tree = StaticMerkleTree::new(path.join("static_merkle_tree")).await?;
        let hasher = create_poseidon_hasher();

        Ok(WithdrawalsDb { list, tree, hasher })
    }

    /// Sets the given withdrawal at the given index in Withdrawals DB.
    ///
    /// # Examples
    ///
    /// Set the given withdrawal at the index 4:
    ///
    /// ```rs
    /// withdrawals_db.set(4, &withdrawal).await?;
    /// ```
    ///
    pub async fn set(&mut self, index: u64, withdrawal: &Withdrawal) -> Result<()> {
        let bytes = withdrawal.to_bytes();
        let fields = withdrawal.to_fields();

        let withdrawal_hash = poseidon_hash(&mut self.hasher, &fields);

        if self.list.get(index).await.is_ok() {
            self.list.set(index, bytes).await?;
        } else {
            self.list.push(bytes).await?;
        }
        self.tree.set_leaf(index, withdrawal_hash).await?;

        Ok(())
    }

    /// Returns the withdrawal at the given index in Withdrawals DB.
    ///
    /// # Examples
    ///
    /// Get the withdrawal at the index 2:
    ///
    /// ```rs
    /// let withdrawal = withdrawals_db.get(2).await?;
    /// ```
    ///
    pub async fn get(&mut self, index: u64) -> Result<Withdrawal> {
        let bytes = self.list.get(index).await?;

        let withdrawal = Withdrawal::from_bytes(&bytes);

        Ok(withdrawal)
    }

    /// Returns the witness of the withdrawal at the given index in Withdrawals DB.
    ///
    /// # Examples
    ///
    /// Get the witness of the withdrawal at the index 3:
    ///
    /// ```rs
    /// let witness = withdrawals_db.get_witness(3).await?;
    /// ```
    ///
    pub async fn get_witness(&mut self, index: u64) -> Result<SingleWithdrawalWitness> {
        let witness = self.tree.get_single_witness(index).await?;

        Ok(witness)
    }

    /// Returns the root hash of Withdrawals DB.
    ///
    /// # Examples
    ///
    /// Get the root hash:
    ///
    /// ```rs
    /// let root = withdrawals_db.get_root().await?;
    /// ```
    ///
    pub async fn get_root(&mut self) -> Result<Field> {
        let root = self.tree.get_root().await?;

        Ok(root)
    }
}

#[cfg(test)]
mod tests {
    use nacho_data_structures::{Address, U256};
    use tokio::fs::remove_dir_all;

    use super::*;

    #[tokio::test]
    async fn creates_withdrawals_db() {
        let dir = "/tmp/nacho/tests/withdrawals_db/creates_withdrawals_db";

        let withdrawals_db = WithdrawalsDb::new(dir).await.unwrap();

        let _ = withdrawals_db;

        remove_dir_all(dir).await.unwrap();
    }

    #[tokio::test]
    async fn sets_and_gets_withdrawals() {
        let dir = "/tmp/nacho/tests/withdrawals_db/sets_and_gets_withdrawals";

        let mut withdrawals_db = WithdrawalsDb::new(dir).await.unwrap();

        let withdrawal_0 = Withdrawal {
            withdrawer: Address::from_bytes(
                "B62qjw5GLgrAZ3U7jWzhTXwnE3URwYmqxDoMzV2P9X1dacY6eJrCm88"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            token_id: U256([0; 32]),
            token_amount: 450,
        };

        let withdrawal_1 = Withdrawal {
            withdrawer: Address::from_bytes(
                "B62qjw5GLgrAZ3U7jWzhTXwnE3URwYmqxDoMzV2P9X1dacY6eJrCm88"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            token_id: U256([1; 32]),
            token_amount: 350,
        };

        let withdrawal_2 = Withdrawal {
            withdrawer: Address::from_bytes(
                "B62qiiGxLsqNemiKFKiD19JdTHmqbE5YKAkMuXGachSdYkTi8xR2dfY"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            token_id: U256([0; 32]),
            token_amount: 250,
        };

        let withdrawal_3 = Withdrawal {
            withdrawer: Address::from_bytes(
                "B62qr1H2QvZVSz7jBEyr91LXFvFTLfHB1W2S9TcMrBiZPHnPQ7yGohY"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            token_id: U256([0; 32]),
            token_amount: 150,
        };

        withdrawals_db.set(0, &withdrawal_0).await.unwrap();
        withdrawals_db.set(1, &withdrawal_1).await.unwrap();
        withdrawals_db.set(2, &withdrawal_2).await.unwrap();
        withdrawals_db.set(3, &withdrawal_3).await.unwrap();

        let updated_withdrawal_2 = Withdrawal {
            token_amount: 32,
            ..withdrawal_2
        };
        withdrawals_db.set(2, &updated_withdrawal_2).await.unwrap();

        assert_eq!(withdrawals_db.get(0).await.unwrap(), withdrawal_0);
        assert_eq!(withdrawals_db.get(1).await.unwrap(), withdrawal_1);
        assert_eq!(withdrawals_db.get(2).await.unwrap(), updated_withdrawal_2);
        assert_eq!(withdrawals_db.get(3).await.unwrap(), withdrawal_3);

        remove_dir_all(dir).await.unwrap();
    }

    #[tokio::test]
    async fn gets_correct_witness() {
        let dir = "/tmp/nacho/tests/withdrawals_db/gets_correct_witness";

        let mut withdrawals_db = WithdrawalsDb::new(dir).await.unwrap();

        let withdrawal_0 = Withdrawal {
            withdrawer: Address::from_bytes(
                "B62qjw5GLgrAZ3U7jWzhTXwnE3URwYmqxDoMzV2P9X1dacY6eJrCm88"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            token_id: U256([0; 32]),
            token_amount: 450,
        };

        withdrawals_db.set(0, &withdrawal_0).await.unwrap();

        let witness = withdrawals_db.get_witness(0).await.unwrap();

        assert_eq!(
            witness
                .siblings
                .iter()
                .map(|sibling| sibling.value.to_fields()[0])
                .collect::<Vec<_>>(),
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
                    .unwrap(),
            ]
        );

        assert_eq!(
            witness
                .siblings
                .iter()
                .map(|sibling| sibling.is_left)
                .collect::<Vec<_>>(),
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false,
            ]
        );

        remove_dir_all(dir).await.unwrap();
    }

    #[tokio::test]
    async fn gets_correct_root() {
        let dir = "/tmp/nacho/tests/withdrawals_db/gets_correct_root";

        let mut withdrawals_db = WithdrawalsDb::new(dir).await.unwrap();

        let root = withdrawals_db.get_root().await.unwrap();

        assert_eq!(
            root,
            "25436453236035485996795240493313170211557120058262356001829805101279552630634"
                .parse()
                .unwrap()
        );

        let withdrawal_0 = Withdrawal {
            withdrawer: Address::from_bytes(
                "B62qjw5GLgrAZ3U7jWzhTXwnE3URwYmqxDoMzV2P9X1dacY6eJrCm88"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            token_id: U256([0; 32]),
            token_amount: 450,
        };

        withdrawals_db.set(0, &withdrawal_0).await.unwrap();

        let root = withdrawals_db.get_root().await.unwrap();

        assert_eq!(
            root,
            "4371168665890896537180909683549128761733587207245536415076358787474804386290"
                .parse()
                .unwrap()
        );

        remove_dir_all(dir).await.unwrap();
    }
}
