use crate::{choose, put_in_order, Sibling};
use data_structures::Field;
use poseidon_hash::{poseidon_hash, PoseidonHasher};

/// The type that represents a Merkle witness of two leaves in a Merkle tree.
///
/// The constant generic parameter `S` is used for number of the siblings and `B` is used for the total size of the witness in bytes.
pub struct WitnessX2<const S: usize, const B: usize> {
    pub(crate) siblings_by_leaves: [[Sibling; S]; 2],
    pub(crate) siblings_at: [bool; S],
}

impl<const S: usize, const B: usize> From<&WitnessX2<S, B>> for [u8; B] {
    fn from(value: &WitnessX2<S, B>) -> Self {
        let mut buf = [0u8; B];

        for j in 0..2 {
            for i in 0..S {
                let pad_start = (i * 33) + (j * 33 * S);
                let pad_end = pad_start + 33;
                let sibling = &value.siblings_by_leaves[j][i];
                let sibling_buf: [u8; 33] = sibling.into();
                buf[pad_start..pad_end].copy_from_slice(&sibling_buf);
            }
        }

        buf
    }
}

impl<const S: usize, const B: usize> WitnessX2<S, B> {
    /// Calculates the root of the Merkle tree the witness represents using the given leaf values.
    pub fn calculate_root(
        &self,
        hasher: &mut PoseidonHasher,
        value_x1: &Field,
        value_x2: &Field,
    ) -> Field {
        let mut root_x1 = value_x1.clone();
        let mut root_x2 = value_x2.clone();

        for i in 0..S {
            let sibling_x1 = choose!(
                self.siblings_at[i],
                root_x2,
                self.siblings_by_leaves[0][i].value
            );

            root_x1 = poseidon_hash(
                hasher,
                put_in_order!(
                    self.siblings_by_leaves[0][i].is_left,
                    &[root_x1, sibling_x1]
                ),
            );

            root_x2 = poseidon_hash(
                hasher,
                put_in_order!(
                    self.siblings_by_leaves[1][i].is_left,
                    &[root_x2, self.siblings_by_leaves[1][i].value]
                ),
            );
        }

        root_x1
    }
}
