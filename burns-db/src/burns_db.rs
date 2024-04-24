use crate::{
    BurnsDbError, SingleBurnWitness, BURNS_TREE_HEIGHT, BURNS_TREE_SIBLING_COUNT,
    BURN_SIZE_IN_BYTES,
};
use nacho_data_structures::{Address, Burn, ByteConversion, Field, FieldConversion, U256};
use nacho_dynamic_list::DynamicList;
use nacho_dynamic_merkle_tree::DynamicMerkleTree;
use nacho_poseidon_hash::{create_poseidon_hasher, poseidon_hash, PoseidonHasher};
use std::{collections::HashMap, path::Path};

type Result<T> = std::result::Result<T, BurnsDbError>;

pub struct BurnsDb {
    list: DynamicList<BURN_SIZE_IN_BYTES>,
    tree: DynamicMerkleTree<BURNS_TREE_HEIGHT, BURNS_TREE_SIBLING_COUNT>,
    indexes: HashMap<Address, Vec<(u64, U256)>>,
    hasher: PoseidonHasher,
}

impl BurnsDb {
    pub async fn new(path: impl AsRef<Path>) -> Result<BurnsDb> {
        let path = path.as_ref();

        let mut list = DynamicList::new(path.join("dynamic_list")).await?;
        let tree = DynamicMerkleTree::new(path.join("dynamic_merkle_tree")).await?;
        let mut indexes = HashMap::<Address, Vec<(u64, U256)>>::new();
        let hasher = create_poseidon_hasher();

        list.for_each(&mut indexes, |buf, index, indexes| {
            let burn = Burn::from_bytes(&buf);

            match indexes.get_mut(&burn.burner) {
                Some(indexes) => {
                    let already_stored = indexes
                        .iter()
                        .any(|(_, token_id)| *token_id == burn.token_id);

                    if already_stored {
                        return Err(BurnsDbError::BurnAlreadyExists);
                    }

                    indexes.push((index, burn.token_id));
                }
                None => {
                    indexes.insert(burn.burner, vec![(index, burn.token_id)]);
                }
            }

            Ok(())
        })
        .await?;

        Ok(BurnsDb {
            list,
            tree,
            indexes,
            hasher,
        })
    }

    pub async fn push(&mut self, burn: &Burn) -> Result<()> {
        let buf = burn.to_bytes();

        match self.indexes.get_mut(&burn.burner) {
            Some(indexes) => {
                let already_stored = indexes
                    .iter()
                    .any(|(_, token_id)| *token_id == burn.token_id);

                if already_stored {
                    return Err(BurnsDbError::BurnAlreadyExists);
                }

                let index = self.list.push(buf).await?;

                indexes.push((index, burn.token_id.clone()));
            }
            None => {
                let index = self.list.push(buf).await?;

                self.indexes
                    .insert(burn.burner.clone(), vec![(index, burn.token_id.clone())]);
            }
        }

        Ok(())
    }

    pub async fn push_leaf(&mut self, burn: &Burn) -> Result<()> {
        let fields = burn.to_fields();

        let hash = poseidon_hash(&mut self.hasher, &fields);

        self.tree.push_leaf(hash).await?;

        Ok(())
    }

    pub async fn get(&mut self, address: &Address, token_id: &U256) -> Result<Burn> {
        let indexes = self
            .indexes
            .get(address)
            .ok_or(BurnsDbError::BurnDoesntExist)?;

        let &(index, _) = indexes
            .iter()
            .find(|(_, f_token_id)| f_token_id == token_id)
            .ok_or(BurnsDbError::BurnDoesntExist)?;

        let buf = self.list.get(index).await?;

        let burn = Burn::from_bytes(&buf);

        Ok(burn)
    }

    pub async fn get_many(&mut self, address: &Address) -> Result<Vec<Burn>> {
        let indexes = self
            .indexes
            .get(address)
            .ok_or(BurnsDbError::BurnDoesntExist)?;

        let mut burns = Vec::with_capacity(indexes.len());

        for &(index, _) in indexes {
            let buf = self.list.get(index).await?;

            let burn = Burn::from_bytes(&buf);

            burns.push(burn)
        }

        Ok(burns)
    }

    pub async fn get_single_witness(
        &mut self,
        address: &Address,
        token_id: &U256,
    ) -> Result<SingleBurnWitness> {
        let indexes = self
            .indexes
            .get(address)
            .ok_or(BurnsDbError::BurnDoesntExist)?;

        let &(index, _) = indexes
            .iter()
            .find(|(_, f_token_id)| f_token_id == token_id)
            .ok_or(BurnsDbError::BurnDoesntExist)?;

        let single_witness = self.tree.get_single_witness(index).await?;

        Ok(single_witness)
    }

    pub async fn get_new_single_witness(&mut self) -> Result<SingleBurnWitness> {
        let single_witness = self.tree.get_unused_single_witness().await?;

        Ok(single_witness)
    }

    pub async fn update(&mut self, burn: &Burn) -> Result<()> {
        let indexes = self
            .indexes
            .get(&burn.burner)
            .ok_or(BurnsDbError::BurnDoesntExist)?;

        let &(index, _) = indexes
            .iter()
            .find(|(_, token_id)| *token_id == burn.token_id)
            .ok_or(BurnsDbError::BurnDoesntExist)?;

        let buf = burn.to_bytes();

        self.list.set(index, buf).await?;

        Ok(())
    }

    pub async fn update_leaf(&mut self, burn: &Burn) -> Result<()> {
        let indexes = self
            .indexes
            .get(&burn.burner)
            .ok_or(BurnsDbError::BurnDoesntExist)?;

        let &(index, _) = indexes
            .iter()
            .find(|(_, f_token_id)| f_token_id == &burn.token_id)
            .ok_or(BurnsDbError::BurnDoesntExist)?;

        let fields = burn.to_fields();

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
    async fn creates_burns_db() {
        let dir = "/tmp/nacho/tests/burns_db/creates_burns_db";

        let burns_db = BurnsDb::new(dir).await.unwrap();

        let _ = burns_db;

        remove_dir_all(dir).await.unwrap();
    }

    #[tokio::test]
    async fn pushes_gets_and_updates_burns_correctly() {
        let dir = "/tmp/nacho/tests/burns_db/pushes_gets_and_updates_burns_correctly";

        let mut burns_db = BurnsDb::new(dir).await.unwrap();

        let burn_1 = Burn {
            burner: Address::from_bytes(
                "B62qjw5GLgrAZ3U7jWzhTXwnE3URwYmqxDoMzV2P9X1dacY6eJrCm88"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            token_id: U256([0; 32]),
            token_amount: 450,
        };

        let mut burn_2 = Burn {
            burner: Address::from_bytes(
                "B62qjw5GLgrAZ3U7jWzhTXwnE3URwYmqxDoMzV2P9X1dacY6eJrCm88"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            token_id: U256([1; 32]),
            token_amount: 350,
        };

        let burn_3 = Burn {
            burner: Address::from_bytes(
                "B62qiiGxLsqNemiKFKiD19JdTHmqbE5YKAkMuXGachSdYkTi8xR2dfY"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            token_id: U256([0; 32]),
            token_amount: 250,
        };

        let burn_4 = Burn {
            burner: Address::from_bytes(
                "B62qr1H2QvZVSz7jBEyr91LXFvFTLfHB1W2S9TcMrBiZPHnPQ7yGohY"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            token_id: U256([0; 32]),
            token_amount: 150,
        };

        let err = burns_db
            .get(&burn_1.burner, &burn_1.token_id)
            .await
            .unwrap_err();

        assert!(matches!(err, BurnsDbError::BurnDoesntExist));

        burns_db.push(&burn_1).await.unwrap();

        let burn = burns_db
            .get(&burn_1.burner, &burn_1.token_id)
            .await
            .unwrap();

        assert_eq!(burn, burn_1);

        let err = burns_db.push(&burn_1).await.unwrap_err();

        assert!(matches!(err, BurnsDbError::BurnAlreadyExists));

        burns_db.push(&burn_2).await.unwrap();

        let burn = burns_db
            .get(&burn_2.burner, &burn_2.token_id)
            .await
            .unwrap();

        assert_eq!(burn, burn_2);

        burns_db.push(&burn_3).await.unwrap();

        let burn = burns_db
            .get(&burn_3.burner, &burn_3.token_id)
            .await
            .unwrap();

        assert_eq!(burn, burn_3);

        let burns = burns_db.get_many(&burn_2.burner).await.unwrap();

        assert_eq!(burns, vec![burn_1, burn_2.clone()]);

        burn_2.token_amount = 100;

        burns_db.update(&burn_2).await.unwrap();

        let burn = burns_db
            .get(&burn_2.burner, &burn_2.token_id)
            .await
            .unwrap();

        assert_eq!(burn, burn_2);

        let err = burns_db.update(&burn_4).await.unwrap_err();

        assert!(matches!(err, BurnsDbError::BurnDoesntExist));

        remove_dir_all(dir).await.unwrap();
    }

    #[tokio::test]
    async fn calculates_correct_roots() {
        let dir = "/tmp/nacho/tests/burns_db/calculates_correct_roots";

        let mut burns_db = BurnsDb::new(dir).await.unwrap();

        let root = burns_db.get_root().await.unwrap();

        assert_eq!(
            root,
            "23937279336243536139305946754911463754843381541673857352836322740025067834219"
                .parse()
                .unwrap()
        );

        let burn = Burn {
            burner: Address::from_bytes(
                "B62qiiGxLsqNemiKFKiD19JdTHmqbE5YKAkMuXGachSdYkTi8xR2dfY"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            token_id: U256([0; 32]),
            token_amount: 250,
        };

        burns_db.push(&burn).await.unwrap();
        burns_db.push_leaf(&burn).await.unwrap();

        let root = burns_db.get_root().await.unwrap();

        assert_eq!(
            root,
            "28479639587013259192146351247697949301975115084093104274892896440875098942098"
                .parse()
                .unwrap()
        );

        let updated_burn = Burn {
            token_amount: 300,
            ..burn
        };

        burns_db.update(&updated_burn).await.unwrap();
        burns_db.update_leaf(&updated_burn).await.unwrap();

        let root = burns_db.get_root().await.unwrap();

        assert_eq!(
            root,
            "2102503854929075198633367373295740762216948253746750778360128509599410644026"
                .parse()
                .unwrap()
        );

        remove_dir_all(dir).await.unwrap();
    }
}
