use crate::BALANCES_TREE_SIBLING_COUNT;
use nacho_merkle_tree::DoubleWitness;

pub type DoubleBalanceWitness = DoubleWitness<BALANCES_TREE_SIBLING_COUNT>;
