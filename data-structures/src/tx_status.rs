/// The enum that represents a transaction's status.
#[derive(PartialEq, Eq, Debug)]
pub enum TxStatus {
    /// The status which means the transaction is rejected.
    Rejected,
    /// The status which means the transaction is pending.
    Pending,
    /// The status which means the transaction is executed.
    Executed,
    /// The status which means a zk proof that includes the transaction is generated.
    Proved,
    /// The status which means the zk proof that includes the transaction is settled to the L1 contract.
    Settled,
}
