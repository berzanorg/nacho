use crate::{
    LiquiditiesDbError, SingleLiquidityWitness, LIQUIDITIES_TREE_HEIGHT,
    LIQUIDITIES_TREE_SIBLING_COUNT, LIQUIDITY_SIZE_IN_BYTES,
};
use nacho_data_structures::{Address, ByteConversion, Field, FieldConversion, Liquidity, U256};
use nacho_dynamic_list::DynamicList;
use nacho_dynamic_merkle_tree::DynamicMerkleTree;
use nacho_poseidon_hash::{create_poseidon_hasher, poseidon_hash, PoseidonHasher};
use std::{collections::HashMap, path::Path};

type Result<T> = std::result::Result<T, LiquiditiesDbError>;

pub struct LiquiditiesDb {
    list: DynamicList<LIQUIDITY_SIZE_IN_BYTES>,
    tree: DynamicMerkleTree<LIQUIDITIES_TREE_HEIGHT, LIQUIDITIES_TREE_SIBLING_COUNT>,
    indexes: HashMap<Address, Vec<(u64, U256, U256)>>,
    hasher: PoseidonHasher,
}

impl LiquiditiesDb {
    pub async fn new(path: impl AsRef<Path>) -> Result<LiquiditiesDb> {
        let path = path.as_ref();

        let mut list = DynamicList::new(path.join("dynamic_list")).await?;
        let tree = DynamicMerkleTree::new(path.join("dynamic_merkle_tree")).await?;
        let mut indexes = HashMap::<Address, Vec<(u64, U256, U256)>>::new();
        let hasher = create_poseidon_hasher();

        list.for_each(&mut indexes, |buf, index, indexes| {
            let liquidity = Liquidity::from_bytes(&buf);

            match indexes.get_mut(&liquidity.provider) {
                Some(indexes) => {
                    let already_stored =
                        indexes.iter().any(|(_, base_token_id, quote_token_id)| {
                            base_token_id == &liquidity.base_token_id
                                && quote_token_id == &liquidity.quote_token_id
                        });

                    if already_stored {
                        return Err(LiquiditiesDbError::LiquidityAlreadyExists);
                    }

                    indexes.push((index, liquidity.base_token_id, liquidity.quote_token_id));
                }
                None => {
                    indexes.insert(
                        liquidity.provider.clone(),
                        vec![(index, liquidity.base_token_id, liquidity.quote_token_id)],
                    );
                }
            }

            Ok(())
        })
        .await?;

        Ok(LiquiditiesDb {
            list,
            tree,
            indexes,
            hasher,
        })
    }

    pub async fn push(&mut self, liquidity: &Liquidity) -> Result<()> {
        let buf = liquidity.to_bytes();

        match self.indexes.get_mut(&liquidity.provider) {
            Some(indexes) => {
                let already_stored = indexes.iter().any(|(_, base_token_id, quote_token_id)| {
                    base_token_id == &liquidity.base_token_id
                        && quote_token_id == &liquidity.quote_token_id
                });

                if already_stored {
                    return Err(LiquiditiesDbError::LiquidityAlreadyExists);
                }

                let index = self.list.push(buf).await?;

                indexes.push((
                    index,
                    liquidity.base_token_id.clone(),
                    liquidity.quote_token_id.clone(),
                ));
            }
            None => {
                let index = self.list.push(buf).await?;

                self.indexes.insert(
                    liquidity.provider.clone(),
                    vec![(
                        index,
                        liquidity.base_token_id.clone(),
                        liquidity.quote_token_id.clone(),
                    )],
                );
            }
        }

        Ok(())
    }

    pub async fn push_leaf(&mut self, liquidity: &Liquidity) -> Result<()> {
        let fields = liquidity.to_fields();

        let hash = poseidon_hash(&mut self.hasher, &fields);

        self.tree.push_leaf(hash).await?;

        Ok(())
    }

    pub async fn get(
        &mut self,
        address: &Address,
        base_token_id: &U256,
        quote_token_id: &U256,
    ) -> Result<Liquidity> {
        let indexes = self
            .indexes
            .get(address)
            .ok_or(LiquiditiesDbError::LiquidityDoesntExist)?;

        let &(index, _, _) = indexes
            .iter()
            .find(|(_, f_base_token_id, f_quote_token_id)| {
                f_base_token_id == base_token_id && f_quote_token_id == quote_token_id
            })
            .ok_or(LiquiditiesDbError::LiquidityDoesntExist)?;

        let buf = self.list.get(index).await?;

        let liquidity = Liquidity::from_bytes(&buf);

        Ok(liquidity)
    }

    pub async fn get_many(&mut self, address: &Address) -> Result<Vec<Liquidity>> {
        let indexes = self
            .indexes
            .get(address)
            .ok_or(LiquiditiesDbError::LiquidityDoesntExist)?;

        let mut liquidities = Vec::with_capacity(indexes.len());

        for &(index, _, _) in indexes {
            let buf = self.list.get(index).await?;

            let liquidity = Liquidity::from_bytes(&buf);

            liquidities.push(liquidity)
        }

        Ok(liquidities)
    }

    pub async fn get_single_witness(
        &mut self,
        address: &Address,
        base_token_id: &U256,
        quote_token_id: &U256,
    ) -> Result<SingleLiquidityWitness> {
        let indexes = self
            .indexes
            .get(address)
            .ok_or(LiquiditiesDbError::LiquidityDoesntExist)?;

        let &(index, _, _) = indexes
            .iter()
            .find(|(_, f_base_token_id, f_quote_token_id)| {
                f_base_token_id == base_token_id && f_quote_token_id == quote_token_id
            })
            .ok_or(LiquiditiesDbError::LiquidityDoesntExist)?;

        let single_witness = self.tree.get_single_witness(index).await?;

        Ok(single_witness)
    }

    pub async fn get_new_single_witness(&mut self) -> Result<SingleLiquidityWitness> {
        let single_witness = self.tree.get_unused_single_witness().await?;

        Ok(single_witness)
    }

    pub async fn update(&mut self, liquidity: &Liquidity) -> Result<()> {
        let indexes = self
            .indexes
            .get(&liquidity.provider)
            .ok_or(LiquiditiesDbError::LiquidityDoesntExist)?;

        let &(index, _, _) = indexes
            .iter()
            .find(|(_, f_base_token_id, f_quote_token_id)| {
                f_base_token_id == &liquidity.base_token_id
                    && f_quote_token_id == &liquidity.quote_token_id
            })
            .ok_or(LiquiditiesDbError::LiquidityDoesntExist)?;

        let buf = liquidity.to_bytes();

        self.list.set(index, buf).await?;

        Ok(())
    }

    pub async fn update_leaf(&mut self, liquidity: &Liquidity) -> Result<()> {
        let indexes = self
            .indexes
            .get(&liquidity.provider)
            .ok_or(LiquiditiesDbError::LiquidityDoesntExist)?;

        let &(index, _, _) = indexes
            .iter()
            .find(|(_, f_base_token_id, f_quote_token_id)| {
                f_base_token_id == &liquidity.base_token_id
                    && f_quote_token_id == &liquidity.quote_token_id
            })
            .ok_or(LiquiditiesDbError::LiquidityDoesntExist)?;

        let fields = liquidity.to_fields();

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
    async fn creates_liquidities_db() {
        let dir = "/tmp/nacho/tests/liquidities_db/creates_liquidities_db";

        let liquidities_db = LiquiditiesDb::new(dir).await.unwrap();

        let _ = liquidities_db;

        remove_dir_all(dir).await.unwrap();
    }

    #[tokio::test]
    async fn pushes_gets_and_updates_liquidities_correctly() {
        let dir = "/tmp/nacho/tests/liquidities_db/pushes_gets_and_updates_liquidities_correctly";

        let mut liquidities_db = LiquiditiesDb::new(dir).await.unwrap();

        let liquidity_1 = Liquidity {
            provider: Address::from_bytes(
                "B62qjw5GLgrAZ3U7jWzhTXwnE3URwYmqxDoMzV2P9X1dacY6eJrCm88"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            base_token_id: U256([0; 32]),
            quote_token_id: U256([1; 32]),
            points: U256([55; 32]),
        };

        let mut liquidity_2 = Liquidity {
            provider: Address::from_bytes(
                "B62qjw5GLgrAZ3U7jWzhTXwnE3URwYmqxDoMzV2P9X1dacY6eJrCm88"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            base_token_id: U256([2; 32]),
            quote_token_id: U256([3; 32]),
            points: U256([22; 32]),
        };

        let liquidity_3 = Liquidity {
            provider: Address::from_bytes(
                "B62qiiGxLsqNemiKFKiD19JdTHmqbE5YKAkMuXGachSdYkTi8xR2dfY"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            base_token_id: U256([0; 32]),
            quote_token_id: U256([1; 32]),
            points: U256([11; 32]),
        };

        let liquidity_4 = Liquidity {
            provider: Address::from_bytes(
                "B62qr1H2QvZVSz7jBEyr91LXFvFTLfHB1W2S9TcMrBiZPHnPQ7yGohY"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            base_token_id: U256([0; 32]),
            quote_token_id: U256([1; 32]),
            points: U256([44; 32]),
        };

        let err = liquidities_db
            .get(
                &liquidity_1.provider,
                &liquidity_1.base_token_id,
                &liquidity_1.quote_token_id,
            )
            .await
            .unwrap_err();

        assert!(matches!(err, LiquiditiesDbError::LiquidityDoesntExist));

        liquidities_db.push(&liquidity_1).await.unwrap();

        let liquidity = liquidities_db
            .get(
                &liquidity_1.provider,
                &liquidity_1.base_token_id,
                &liquidity_1.quote_token_id,
            )
            .await
            .unwrap();

        assert_eq!(liquidity, liquidity_1);

        let err = liquidities_db.push(&liquidity_1).await.unwrap_err();

        assert!(matches!(err, LiquiditiesDbError::LiquidityAlreadyExists));

        liquidities_db.push(&liquidity_2).await.unwrap();

        let liquidity = liquidities_db
            .get(
                &liquidity_2.provider,
                &liquidity_2.base_token_id,
                &liquidity_2.quote_token_id,
            )
            .await
            .unwrap();

        assert_eq!(liquidity, liquidity_2);

        liquidities_db.push(&liquidity_3).await.unwrap();

        let liquidity = liquidities_db
            .get(
                &liquidity_3.provider,
                &liquidity_3.base_token_id,
                &liquidity_3.quote_token_id,
            )
            .await
            .unwrap();

        assert_eq!(liquidity, liquidity_3);

        let liquidities = liquidities_db
            .get_many(&liquidity_2.provider)
            .await
            .unwrap();

        assert_eq!(liquidities, vec![liquidity_1, liquidity_2.clone()]);

        liquidity_2.points = U256([77; 32]);

        liquidities_db.update(&liquidity_2).await.unwrap();

        let liquidity = liquidities_db
            .get(
                &liquidity_2.provider,
                &liquidity_2.base_token_id,
                &liquidity_2.quote_token_id,
            )
            .await
            .unwrap();

        assert_eq!(liquidity, liquidity_2);

        let err = liquidities_db.update(&liquidity_4).await.unwrap_err();

        assert!(matches!(err, LiquiditiesDbError::LiquidityDoesntExist));

        remove_dir_all(dir).await.unwrap();
    }

    #[tokio::test]
    async fn calculates_correct_roots() {
        let dir = "/tmp/nacho/tests/liquidities_db/calculates_correct_roots";

        let mut liquidities_db = LiquiditiesDb::new(dir).await.unwrap();

        let root = liquidities_db.get_root().await.unwrap();

        assert_eq!(
            root,
            "1945127946440409282447574121167141731006841597528804291507158560727071219394"
                .parse()
                .unwrap()
        );

        let balance = Liquidity {
            provider: Address::from_bytes(
                "B62qr1H2QvZVSz7jBEyr91LXFvFTLfHB1W2S9TcMrBiZPHnPQ7yGohY"
                    .as_bytes()
                    .try_into()
                    .unwrap(),
            ),
            base_token_id: U256([0; 32]),
            quote_token_id: U256([1; 32]),
            points: U256([44; 32]),
        };

        liquidities_db.push(&balance).await.unwrap();
        liquidities_db.push_leaf(&balance).await.unwrap();

        let root = liquidities_db.get_root().await.unwrap();

        assert_eq!(
            root,
            "13548228949217198807310792093561457131520929862298447192231975208087040415890"
                .parse()
                .unwrap()
        );

        let updated_liquidity = Liquidity {
            points: U256([35; 32]),
            ..balance
        };

        liquidities_db.update(&updated_liquidity).await.unwrap();
        liquidities_db
            .update_leaf(&updated_liquidity)
            .await
            .unwrap();

        let root = liquidities_db.get_root().await.unwrap();

        assert_eq!(
            root,
            "20321691250697101313976472915314419977063523481388894706218314657377909871012"
                .parse()
                .unwrap()
        );

        remove_dir_all(dir).await.unwrap();
    }
}
