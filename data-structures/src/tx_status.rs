use crate::ByteConversion;

/// The enum that represents a transaction's status.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxStatus {
    /// The status which means the transaction is pending.
    Pending,
    /// The status which means the transaction is rejected.
    Rejected,
    /// The status which means the transaction is executed.
    Executed,
    /// The status which means a zk proof that includes the transaction is generated.
    Proved,
    /// The status which means the zk proof that includes the transaction is settled to the L1 contract.
    Settled,
}

impl ByteConversion<1> for TxStatus {
    fn to_bytes(&self) -> [u8; 1] {
        [self.to_owned() as u8]
    }

    fn from_bytes(bytes: &[u8; 1]) -> Self {
        match bytes[0] {
            0 => Self::Pending,
            1 => Self::Rejected,
            2 => Self::Executed,
            3 => Self::Proved,
            _ => Self::Settled,
        }
    }
}
