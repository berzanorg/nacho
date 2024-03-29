mod burns_db;
mod constants;
mod error;
mod single_burn_witness;

pub use burns_db::BurnsDb;
pub use constants::{BURNS_TREE_HEIGHT, BURNS_TREE_SIBLING_COUNT, BURN_SIZE_IN_BYTES};
pub use error::BurnsDbError;
pub use single_burn_witness::SingleBurnWitness;
