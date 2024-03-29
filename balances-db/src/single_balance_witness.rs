use crate::BALANCES_TREE_SIBLING_COUNT;
use nacho_merkle_tree::SingleWitness;

pub type SingleBalanceWitness = SingleWitness<BALANCES_TREE_SIBLING_COUNT>;
