use crate::WITHDRAWALS_TREE_SIBLING_COUNT;
use nacho_data_structures::SingleMerkleWitness;

pub type SingleWithdrawalWitness = SingleMerkleWitness<WITHDRAWALS_TREE_SIBLING_COUNT>;
