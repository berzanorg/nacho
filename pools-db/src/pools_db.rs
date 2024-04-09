use crate::{
    PoolsDbError, SinglePoolWitness, POOLS_TREE_HEIGHT, POOLS_TREE_SIBLING_COUNT,
    POOL_SIZE_IN_BYTES,
};
use nacho_data_structures::{ByteConversion, FieldConversion, Pool, U256};
use nacho_dynamic_list::DynamicList;
use nacho_merkle_tree::MerkleTree;
use nacho_poseidon_hash::{create_poseidon_hasher, poseidon_hash, PoseidonHasher};
use std::{collections::HashMap, path::Path};

type Result<T> = std::result::Result<T, PoolsDbError>;

pub struct PoolsDb {
    list: DynamicList<POOL_SIZE_IN_BYTES>,
    tree: MerkleTree<POOLS_TREE_HEIGHT, POOLS_TREE_SIBLING_COUNT>,
    indexes: HashMap<(U256, U256), u64>,
    hasher: PoseidonHasher,
}

impl PoolsDb {
    pub async fn new(path: impl AsRef<Path>) -> Result<PoolsDb> {
        let path = path.as_ref();

        let mut list = DynamicList::new(path.join("dynamic_list")).await?;
        let tree = MerkleTree::new(path.join("merkle_tree")).await?;
        let mut indexes = HashMap::<(U256, U256), u64>::new();
        let hasher = create_poseidon_hasher();

        list.for_each(&mut indexes, |buf, index, indexes| {
            let pool = Pool::from_bytes(&buf);

            if indexes.contains_key(&(pool.base_token_id.clone(), pool.quote_token_id.clone())) {
                return Err(PoolsDbError::PoolAlreadyExists);
            }

            indexes.insert((pool.base_token_id, pool.quote_token_id), index);

            Ok(())
        })
        .await?;

        Ok(PoolsDb {
            list,
            tree,
            indexes,
            hasher,
        })
    }

    pub async fn push(&mut self, pool: &Pool) -> Result<()> {
        let buf = pool.to_bytes();

        if self
            .indexes
            .contains_key(&(pool.base_token_id.clone(), pool.quote_token_id.clone()))
        {
            return Err(PoolsDbError::PoolAlreadyExists);
        }

        let index = self.list.push(buf).await?;

        self.indexes.insert(
            (pool.base_token_id.clone(), pool.quote_token_id.clone()),
            index,
        );

        Ok(())
    }

    pub async fn push_leaf(&mut self, pool: &Pool) -> Result<()> {
        let fields = pool.to_fields();

        let hash = poseidon_hash(&mut self.hasher, &fields);

        self.tree.push(hash).await?;

        Ok(())
    }

    pub async fn get(&mut self, base_token_id: &U256, quote_token_id: &U256) -> Result<Pool> {
        let &index = self
            .indexes
            .get(&(base_token_id.clone(), quote_token_id.clone()))
            .ok_or(PoolsDbError::PoolDoesntExist)?;

        let buf = self.list.get(index).await?;

        let pool = Pool::from_bytes(&buf);

        Ok(pool)
    }

    pub async fn get_many(&mut self) -> Result<Vec<Pool>> {
        let mut indexes: Vec<_> = self.indexes.values().collect();
        indexes.sort();

        let mut pools = Vec::with_capacity(indexes.len());

        for &index in indexes {
            let buf = self.list.get(index).await?;

            let pool = Pool::from_bytes(&buf);

            pools.push(pool)
        }

        Ok(pools)
    }

    pub async fn get_single_witness(
        &mut self,
        base_token_id: &U256,
        quote_token_id: &U256,
    ) -> Result<SinglePoolWitness> {
        let &index = self
            .indexes
            .get(&(base_token_id.clone(), quote_token_id.clone()))
            .ok_or(PoolsDbError::PoolDoesntExist)?;

        let single_witness = self.tree.get_single_witness(index).await?;

        Ok(single_witness)
    }

    pub async fn get_new_single_witness(&mut self) -> Result<SinglePoolWitness> {
        let single_witness = self.tree.get_new_single_witness().await?;

        Ok(single_witness)
    }

    pub async fn update(&mut self, pool: &Pool) -> Result<()> {
        let &index = self
            .indexes
            .get(&(pool.base_token_id.clone(), pool.quote_token_id.clone()))
            .ok_or(PoolsDbError::PoolDoesntExist)?;

        let buf = pool.to_bytes();

        self.list.set(index, buf).await?;

        Ok(())
    }

    pub async fn update_leaf(&mut self, pool: &Pool) -> Result<()> {
        let &index = self
            .indexes
            .get(&(pool.base_token_id.clone(), pool.quote_token_id.clone()))
            .ok_or(PoolsDbError::PoolDoesntExist)?;

        let fields = pool.to_fields();

        let hash = poseidon_hash(&mut self.hasher, &fields);

        self.tree.set(index, hash).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tokio::fs::remove_dir_all;

    use super::*;

    #[tokio::test]
    async fn creates_pools_db() {
        let dir = "/tmp/nacho/tests/pools_db/creates_pools_db";

        let pools_db = PoolsDb::new(dir).await.unwrap();

        let _ = pools_db;

        remove_dir_all(dir).await.unwrap();
    }

    #[tokio::test]
    async fn pushes_gets_and_updates_pools_correctly() {
        let dir = "/tmp/nacho/tests/pools_db/pushes_gets_and_updates_pools_correctly";

        let mut pools_db = PoolsDb::new(dir).await.unwrap();

        let pool_1 = Pool {
            base_token_id: U256([0; 32]),
            quote_token_id: U256([1; 32]),
            base_token_amount: 4000,
            quote_token_amount: 8000,
            total_liqudity_points: U256([88; 32]),
        };

        let mut pool_2 = Pool {
            base_token_id: U256([2; 32]),
            quote_token_id: U256([3; 32]),
            base_token_amount: 2500,
            quote_token_amount: 3000,
            total_liqudity_points: U256([66; 32]),
        };

        let pool_3 = Pool {
            base_token_id: U256([0; 32]),
            quote_token_id: U256([1; 32]),
            base_token_amount: 5000,
            quote_token_amount: 9000,
            total_liqudity_points: U256([55; 32]),
        };

        let pool_4 = Pool {
            base_token_id: U256([3; 32]),
            quote_token_id: U256([0; 32]),
            base_token_amount: 3000,
            quote_token_amount: 6000,
            total_liqudity_points: U256([33; 32]),
        };

        let err = pools_db
            .get(&pool_1.base_token_id, &pool_1.quote_token_id)
            .await
            .unwrap_err();

        assert!(matches!(err, PoolsDbError::PoolDoesntExist));

        pools_db.push(&pool_1).await.unwrap();

        let pool = pools_db
            .get(&pool_1.base_token_id, &pool_1.quote_token_id)
            .await
            .unwrap();

        assert_eq!(pool, pool_1);

        let err = pools_db.push(&pool_1).await.unwrap_err();

        assert!(matches!(err, PoolsDbError::PoolAlreadyExists));

        pools_db.push(&pool_2).await.unwrap();

        let pool = pools_db
            .get(&pool_2.base_token_id, &pool_2.quote_token_id)
            .await
            .unwrap();

        assert_eq!(pool, pool_2);

        let err = pools_db.push(&pool_3).await.unwrap_err();

        assert!(matches!(err, PoolsDbError::PoolAlreadyExists));

        let pools = pools_db.get_many().await.unwrap();

        assert_eq!(pools, vec![pool_1, pool_2.clone()]);

        pool_2.total_liqudity_points = U256([77; 32]);

        pools_db.update(&pool_2).await.unwrap();

        let pool = pools_db
            .get(&pool_2.base_token_id, &pool_2.quote_token_id)
            .await
            .unwrap();

        assert_eq!(pool, pool_2);

        let err = pools_db.update(&pool_4).await.unwrap_err();

        assert!(matches!(err, PoolsDbError::PoolDoesntExist));

        remove_dir_all(dir).await.unwrap();
    }
}
