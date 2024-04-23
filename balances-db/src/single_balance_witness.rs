use crate::BALANCES_TREE_SIBLING_COUNT;
use nacho_data_structures::SingleMerkleWitness;

pub type SingleBalanceWitness = SingleMerkleWitness<BALANCES_TREE_SIBLING_COUNT>;
