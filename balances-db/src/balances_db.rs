use crate::{
    BalancesDbError, DoubleBalanceWitness, SingleBalanceWitness, BALANCES_TREE_HEIGHT,
    BALANCES_TREE_SIBLING_COUNT, BALANCE_SIZE_IN_BYTES,
};
use nacho_data_structures::{Address, Balance, ByteConversion, Field, FieldConversion, U256};
use nacho_dynamic_list::DynamicList;
use nacho_dynamic_merkle_tree::DynamicMerkleTree;
use nacho_poseidon_hash::{create_poseidon_hasher, poseidon_hash, PoseidonHasher};
use std::{collections::HashMap, path::Path};

type Result<T> = std::result::Result<T, BalancesDbError>;

pub struct BalancesDb {
    list: DynamicList<BALANCE_SIZE_IN_BYTES>,
    tree: DynamicMerkleTree<BALANCES_TREE_HEIGHT, BALANCES_TREE_SIBLING_COUNT>,
    indexes: HashMap<Address, Vec<(u64, U256)>>,
    hasher: PoseidonHasher,
}

impl BalancesDb {
    pub async fn new(path: impl AsRef<Path>) -> Result<BalancesDb> {
        let path = path.as_ref();

        let mut list = DynamicList::new(path.join("dynamic_list")).await?;
        let tree = DynamicMerkleTree::new(path.join("dynamic_merkle_tree")).await?;
        let mut indexes = HashMap::<Address, Vec<(u64, U256)>>::new();
        let hasher = create_poseidon_hasher();

        list.for_each(&mut indexes, |buf, index, indexes| {
            let balance = Balance::from_bytes(&buf);

            match indexes.get_mut(&balance.owner) {
                Some(indexes) => {
                    let already_stored = indexes
                        .iter()
                        .any(|(_, token_id)| *token_id == balance.token_id);

                    if already_stored {
                        return Err(BalancesDbError::BalanceAlreadyExists);
                    }

                    indexes.push((index, balance.token_id));
                }
                None => {
                    indexes.insert(balance.owner, vec![(index, balance.token_id)]);
                }
            }

            Ok(())
        })
        .await?;

        Ok(BalancesDb {
            list,
            tree,
            indexes,
            hasher,
        })
    }

    pub async fn push(&mut self, balance: &Balance) -> Result<()> {
        let buf = balance.to_bytes();

        match self.indexes.get_mut(&balance.owner) {
            Some(indexes) => {
                let already_stored = indexes
                    .iter()
                    .any(|(_, token_id)| *token_id == balance.token_id);

                if already_stored {
                    return Err(BalancesDbError::BalanceAlreadyExists);
                }

                let index = self.list.push(buf).await?;

                indexes.push((index, balance.token_id.clone()));
            }
            None => {
                let index = self.list.push(buf).await?;

                self.indexes.insert(
                    balance.owner.clone(),
                    vec![(index, balance.token_id.clone())],
                );
            }
        }

        Ok(())
    }

    pub async fn push_leaf(&mut self, balance: &Balance) -> Result<()> {
        let fields = balance.to_fields();

        let hash = poseidon_hash(&mut self.hasher, &fields);

        self.tree.push_leaf(hash).await?;

        Ok(())
    }

    pub async fn get(&mut self, address: &Address, token_id: &U256) -> Result<Balance> {
        match self.indexes.get(address) {
            Some(indexes) => {
                let &(index, _) = indexes
                    .iter()
                    .find(|(_, f_token_id)| f_token_id == token_id)
                    .ok_or(BalancesDbError::BalanceDoesntExist)?;

                let buf = self.list.get(index).await?;

                let balance = Balance::from_bytes(&buf);

                Ok(balance)
            }
            None => Err(BalancesDbError::BalanceDoesntExist),
        }
    }

    pub async fn get_many(&mut self, address: &Address) -> Result<Vec<Balance>> {
        match self.indexes.get(address) {
            Some(indexes) => {
                let mut balances = Vec::with_capacity(indexes.len());

                for &(index, _) in indexes {
                    let buf = self.list.get(index).await?;

                    let balance = Balance::from_bytes(&buf);

                    balances.push(balance)
                }

                Ok(balances)
            }
            None => Err(BalancesDbError::BalanceDoesntExist),
        }
    }

    pub async fn get_single_witness(
        &mut self,
        address: &Address,
        token_id: &U256,
    ) -> Result<SingleBalanceWitness> {
        let indexes = self
            .indexes
            .get(address)
            .ok_or(BalancesDbError::BalanceDoesntExist)?;

        let &(index, _) = indexes
            .iter()
            .find(|(_, f_token_id)| f_token_id == token_id)
            .ok_or(BalancesDbError::BalanceDoesntExist)?;

        let single_witness = self.tree.get_single_witness(index).await?;

        Ok(single_witness)
    }

    pub async fn get_double_witness(
        &mut self,
        address_x1: &Address,
        token_id_x1: &U256,
        address_x2: &Address,
        token_id_x2: &U256,
    ) -> Result<DoubleBalanceWitness> {
        let indexes_x1 = self
            .indexes
            .get(address_x1)
            .ok_or(BalancesDbError::BalanceDoesntExist)?;

        let &(index_x1, _) = indexes_x1
            .iter()
            .find(|(_, f_token_id)| f_token_id == token_id_x1)
            .ok_or(BalancesDbError::BalanceDoesntExist)?;

        let indexes_x2 = self
            .indexes
            .get(address_x2)
            .ok_or(BalancesDbError::BalanceDoesntExist)?;

        let &(index_x2, _) = indexes_x2
            .iter()
            .find(|(_, f_token_id)| f_token_id == token_id_x2)
            .ok_or(BalancesDbError::BalanceDoesntExist)?;

        let double_witness = self.tree.get_double_witness(index_x1, index_x2).await?;

        Ok(double_witness)
    }

    pub async fn get_new_single_witness(&mut self) -> Result<SingleBalanceWitness> {
        let single_witness = self.tree.get_unused_single_witness().await?;

        Ok(single_witness)
    }

    pub async fn update(&mut self, balance: &Balance) -> Result<()> {
        let indexes = self
            .indexes
            .get(&balance.owner)
            .ok_or(BalancesDbError::BalanceDoesntExist)?;

        let &(index, _) = indexes
            .iter()
            .find(|(_, token_id)| *token_id == balance.token_id)
            .ok_or(BalancesDbError::BalanceDoesntExist)?;

        let buf = balance.to_bytes();

        self.list.set(index, buf).await?;

        Ok(())
    }

    pub async fn update_leaf(&mut self, balance: &Balance) -> Result<()> {
        let indexes = self
            .indexes
            .get(&balance.owner)
            .ok_or(BalancesDbError::BalanceDoesntExist)?;

        let &(index, _) = indexes
            .iter()
            .find(|(_, f_token_id)| f_token_id == &balance.token_id)
            .ok_or(BalancesDbError::BalanceDoesntExist)?;

        let fields = balance.to_fields();

        let hash = poseidon_hash(&mut self.hasher, &fields);

        self.tree.set_leaf(index, hash).await?;

        Ok(())
    }

    pub async fn get_root(&mut self) -> Result<Field> {
        let root = self.tree.get_root().await?;

        Ok(root)
    }
}

#[cfg(test)]
mod tests {
    use tokio::fs::remove_dir_all;

    use super::*;

    #[tokio::test]
    async fn creates_balances_db() {
        let dir = "/tmp/nacho/tests/balances_db/creates_balances_db";

        let balances_db = BalancesDb::new(dir).await.unwrap();

        let _ = balances_db;

        remove_dir_all(dir).await.unwrap();
    }

    #[tokio::test]
    async fn pushes_gets_and_updates_balances_correctly() {
        let dir = "/tmp/nacho/tests/balances_db/pushes_gets_and_updates_balances_correctly";

        let mut balances_db = BalancesDb::new(dir).await.unwrap();

        let balance_1 = Balance {
            owner: Address::from_bytes(
                "B62qjw5GLgrAZ3U7jWzhTXwnE3URwYmqxDoMzV2P9X1dacY6eJrCm88"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            token_id: U256([0; 32]),
            token_amount: 450,
        };

        let mut balance_2 = Balance {
            owner: Address::from_bytes(
                "B62qjw5GLgrAZ3U7jWzhTXwnE3URwYmqxDoMzV2P9X1dacY6eJrCm88"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            token_id: U256([1; 32]),
            token_amount: 350,
        };

        let balance_3 = Balance {
            owner: Address::from_bytes(
                "B62qiiGxLsqNemiKFKiD19JdTHmqbE5YKAkMuXGachSdYkTi8xR2dfY"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            token_id: U256([0; 32]),
            token_amount: 250,
        };

        let balance_4 = Balance {
            owner: Address::from_bytes(
                "B62qr1H2QvZVSz7jBEyr91LXFvFTLfHB1W2S9TcMrBiZPHnPQ7yGohY"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            token_id: U256([0; 32]),
            token_amount: 150,
        };

        let err = balances_db
            .get(&balance_1.owner, &balance_1.token_id)
            .await
            .unwrap_err();

        assert!(matches!(err, BalancesDbError::BalanceDoesntExist));

        balances_db.push(&balance_1).await.unwrap();

        let balance = balances_db
            .get(&balance_1.owner, &balance_1.token_id)
            .await
            .unwrap();

        assert_eq!(balance, balance_1);

        let err = balances_db.push(&balance_1).await.unwrap_err();

        assert!(matches!(err, BalancesDbError::BalanceAlreadyExists));

        balances_db.push(&balance_2).await.unwrap();

        let balance = balances_db
            .get(&balance_2.owner, &balance_2.token_id)
            .await
            .unwrap();

        assert_eq!(balance, balance_2);

        balances_db.push(&balance_3).await.unwrap();

        let balance = balances_db
            .get(&balance_3.owner, &balance_3.token_id)
            .await
            .unwrap();

        assert_eq!(balance, balance_3);

        let balances = balances_db.get_many(&balance_2.owner).await.unwrap();

        assert_eq!(balances, vec![balance_1, balance_2.clone()]);

        balance_2.token_amount = 100;

        balances_db.update(&balance_2).await.unwrap();

        let balance = balances_db
            .get(&balance_2.owner, &balance_2.token_id)
            .await
            .unwrap();

        assert_eq!(balance, balance_2);

        let err = balances_db.update(&balance_4).await.unwrap_err();

        assert!(matches!(err, BalancesDbError::BalanceDoesntExist));

        remove_dir_all(dir).await.unwrap();
    }

    #[tokio::test]
    async fn calculates_correct_roots() {
        let dir = "/tmp/nacho/tests/balances_db/calculates_correct_roots";

        let mut balances_db = BalancesDb::new(dir).await.unwrap();

        let root = balances_db.get_root().await.unwrap();

        assert_eq!(
            root,
            "27841935691558593279858640177961574373148122335514448527568736064618172266482"
                .parse()
                .unwrap()
        );

        let balance = Balance {
            owner: Address::from_bytes(
                "B62qr1H2QvZVSz7jBEyr91LXFvFTLfHB1W2S9TcMrBiZPHnPQ7yGohY"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            token_id: U256([0; 32]),
            token_amount: 150,
        };

        balances_db.push(&balance).await.unwrap();
        balances_db.push_leaf(&balance).await.unwrap();

        let root = balances_db.get_root().await.unwrap();

        assert_eq!(
            root,
            "27489112645307945006783215842707596600256272842068372142876299295747751229490"
                .parse()
                .unwrap()
        );

        let updated_balance = Balance {
            token_amount: 90,
            ..balance
        };

        balances_db.update(&updated_balance).await.unwrap();
        balances_db.update_leaf(&updated_balance).await.unwrap();

        let root = balances_db.get_root().await.unwrap();

        assert_eq!(
            root,
            "3847128151157624127835942057282164084040902825349057573274273979399571932032"
                .parse()
                .unwrap()
        );

        remove_dir_all(dir).await.unwrap();
    }
}
