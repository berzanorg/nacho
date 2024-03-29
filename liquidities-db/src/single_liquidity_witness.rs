use crate::LIQUIDITIES_TREE_SIBLING_COUNT;
use nacho_merkle_tree::SingleWitness;

pub type SingleLiquidityWitness = SingleWitness<LIQUIDITIES_TREE_SIBLING_COUNT>;
