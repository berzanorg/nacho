use crate::{put_in_order, Sibling};
use data_structures::Field;
use poseidon_hash::{poseidon_hash, PoseidonHasher};

/// The type that represents a Merkle witness of a leaf in a Merkle tree.
///
/// The constant generic parameter `S` is used for number of the siblings and `B` is used for the total size of the witness in bytes.
pub struct WitnessX1<const S: usize, const B: usize> {
    pub siblings: [Sibling; S],
}

impl<const S: usize, const B: usize> From<&WitnessX1<S, B>> for [u8; B] {
    fn from(value: &WitnessX1<S, B>) -> Self {
        let mut buf = [0u8; B];

        for i in 0..S {
            let pad_start = i * 33;
            let pad_end = pad_start + 33;
            let sibling = &value.siblings[i];
            let sibling_buf: [u8; 33] = sibling.into();
            buf[pad_start..pad_end].copy_from_slice(&sibling_buf);
        }

        buf
    }
}

impl<const S: usize, const B: usize> WitnessX1<S, B> {
    /// Calculates the root of the Merkle tree the witness represents using the given leaf value.
    pub fn calculate_root(&self, hasher: &mut PoseidonHasher, value: &Field) -> Field {
        let mut root = value.clone();

        for i in 0..S {
            root = poseidon_hash(
                hasher,
                put_in_order!(self.siblings[i].is_left, &[root, self.siblings[i].value]),
            );
        }

        root
    }
}
