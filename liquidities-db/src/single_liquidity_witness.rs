use crate::LIQUIDITIES_TREE_SIBLING_COUNT;
use nacho_data_structures::SingleMerkleWitness;

pub type SingleLiquidityWitness = SingleMerkleWitness<LIQUIDITIES_TREE_SIBLING_COUNT>;
