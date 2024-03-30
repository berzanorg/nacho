use crate::WITHDRAWALS_TREE_SIBLING_COUNT;
use nacho_merkle_tree::SingleWitness;

pub type SingleWithdrawalWitness = SingleWitness<WITHDRAWALS_TREE_SIBLING_COUNT>;
