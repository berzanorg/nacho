use crate::{pick, MerkleTreeError, Sibling, WitnessX1, WitnessX2, WitnessX3, WitnessX4};
use data_structures::{field_from_bytes, field_to_bytes, Field};
use poseidon_hash::{create_poseidon_hasher, poseidon_hash, PoseidonHasher};
use std::{io::SeekFrom, path::Path};
use tokio::{
    fs::{create_dir_all, File, OpenOptions},
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
};

type Result<T> = std::result::Result<T, MerkleTreeError>;

/// An on disk Merkle tree implementation optimized for low storage and constant memory usage.
///
/// The constant generic parameter `H` represents the height of the Merkle tree.
///
/// Data of each height is represented by its own dedicated file.
///
/// Hash of zeroes are calculated and cached during the initialization.
///
/// # Examples
///
/// Create an on disk Merkle tree at the given path with 42 as the height:
///
/// ```rs
/// let tree = MerkleTree::<42>::new("/tmp/nacho/tree").await?;
/// ```
///
/// Assign the value 42 to the leaf at the 7th index:
///
/// ```rs
/// tree.set(7, Field::from(42)).await?;
/// ```
///
/// Push the value 7 to the Merkle tree and get the index pushed to:
///
/// ```rs
/// let index = tree.push(7.into()).await?;
/// ```
///
/// Get the value of the leaf at the 7th index:
///
/// ```rs
/// let value = tree.get(7).await?;
/// ```
///
/// Get the root:
///
/// ```rs
/// let root = tree.root().await?;
/// ```
///
/// Get the witness of the leaf at the 7th index:
///
/// ```rs
/// let witness = tree.witness(7).await?;
/// ```
///
pub struct MerkleTree<
    const H: usize,
    const S: usize,
    const A: usize,
    const B: usize,
    const C: usize,
    const D: usize,
> {
    files: [File; H],
    zeroes: [Field; H],
    hasher: PoseidonHasher,
}

impl<
        const H: usize,
        const S: usize,
        const A: usize,
        const B: usize,
        const C: usize,
        const D: usize,
    > MerkleTree<H, S, A, B, C, D>
{
    /// The maximum number of leaves in the Merkle tree.
    const MAX_NUMBER_OF_LEAVES: u64 = u64::pow(2, H as u32 - 1);

    /// The maximum number of leaves in the Merkle tree.
    const MAX_INDEX: u64 = Self::MAX_NUMBER_OF_LEAVES - 1;

    /// Creates a new `MerkleTree` at the given path.
    ///
    /// The constant generic parameter `H` is used to set the height of the Merkle tree.
    ///
    /// All data is stored in files inside the folder at the given path and they shouldn't be edited manually.
    ///
    /// # Examples
    ///
    /// Create a new Merkle tree with 42 as the height:
    ///
    /// ```rs
    /// let tree = MerkleTree::<42>::new().await?;
    /// ```
    ///
    pub async fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        create_dir_all(path).await?;

        let mut files = vec![];

        for h in 0..H {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path.join(h.to_string()))
                .await?;

            files.push(file)
        }

        let files = files.try_into().map_err(|_| MerkleTreeError::Infallible)?;

        let zero: Field = 0.into();
        let mut zeroes = [zero; H];

        let mut hasher = create_poseidon_hasher();

        for i in 1..H {
            zeroes[i] = poseidon_hash(&mut hasher, &[zeroes[i - 1], zeroes[i - 1]]);
        }

        Ok(Self {
            files,
            zeroes,
            hasher,
        })
    }

    /// Returns the root of the Merkle tree.
    ///
    /// # Examples
    ///
    /// Get the root:
    ///
    /// ```rs
    /// let root = tree.root().await?;
    /// ```
    ///
    pub async fn root(&mut self) -> Result<Field> {
        let leaves_file_len = self.files[0].metadata().await?.len();
        let root_file = &mut self.files[H - 1];

        if leaves_file_len == 0 {
            return Ok(self.zeroes[H - 1]);
        }

        let mut buf = [0; 32];

        root_file.seek(SeekFrom::Start(0)).await?;
        root_file.read_exact(&mut buf).await?;

        Ok(field_from_bytes(&buf))
    }

    /// Returns the value of the leaf at the given index of the Merkle tree.
    ///
    /// Returns zero for indexes that are not used yet.
    ///
    /// # Examples
    ///
    /// Get the value of the leaf at the 7th index:
    ///
    /// ```rs
    /// let value = tree.get(7).await?;
    /// ```
    ///
    pub async fn get(&mut self, index: u64) -> Result<Field> {
        if index >= Self::MAX_NUMBER_OF_LEAVES {
            return Err(MerkleTreeError::NonExistentIndex(Self::MAX_INDEX));
        }

        let leaves_file = &mut self.files[0];

        let leaves_file_len = leaves_file.metadata().await?.len();
        let padding = index * 32;
        let mut buf = [0; 32];

        if leaves_file_len <= padding {
            return Ok(field_from_bytes(&buf));
        }

        leaves_file.seek(SeekFrom::Start(padding)).await?;
        leaves_file.read_exact(&mut buf).await?;

        Ok(field_from_bytes(&buf))
    }

    /// Assigns the given value to the leaf at the given index of the Merkle tree.
    ///
    /// # Examples
    ///
    /// Assign the value 42 to the leaf at the 7th index:
    ///
    /// ```rs
    /// tree.set(7, Field::from(42)).await?;
    /// ```
    ///
    pub async fn set(&mut self, index: u64, value: Field) -> Result<()> {
        if index > Self::MAX_INDEX {
            return Err(MerkleTreeError::NonExistentIndex(Self::MAX_INDEX));
        }

        let leaves_file_len = {
            let file = &mut self.files[0];
            file.metadata().await?.len()
        };

        let padding = index * 32;

        if padding > leaves_file_len {
            return Err(MerkleTreeError::UnusableIndex(leaves_file_len / 32));
        }

        let mut current_value = value;
        let mut current_padding = padding;

        for j in 0..H - 1 {
            let current_file = &mut self.files[j];

            let parent_index = index / 2_u64.pow(j as u32 + 1);

            let sibling_index = {
                let t = index / 2_u64.pow(j as u32);
                if t % 2 == 0 {
                    t + 1
                } else {
                    t - 1
                }
            };

            let sibling_padding = sibling_index * 32;
            let sibling_is_left = sibling_index % 2 == 0;
            let file_len = current_file.metadata().await?.len();

            let sibling_value = if file_len == 0 || sibling_padding > file_len - 32 {
                self.zeroes[j]
            } else {
                let mut buf = [0_u8; 32];
                current_file.seek(SeekFrom::Start(sibling_padding)).await?;
                current_file.read_exact(&mut buf).await?;
                field_from_bytes(&buf)
            };

            let (left, right) = if sibling_is_left {
                (sibling_value, current_value)
            } else {
                (current_value, sibling_value)
            };

            let parent_value = poseidon_hash(&mut self.hasher, &[left, right]);

            current_file.seek(SeekFrom::Start(current_padding)).await?;
            current_file.write(&field_to_bytes(&current_value)).await?;
            current_file.flush().await?;

            let next_file = &mut self.files[j + 1];

            let next_padding = parent_index * 32;

            next_file.seek(SeekFrom::Start(next_padding)).await?;
            next_file.write(&field_to_bytes(&parent_value)).await?;
            next_file.flush().await?;

            current_value = parent_value;
            current_padding = next_padding;
        }

        Ok(())
    }

    /// Assigns the given value to the leaf at the latest unused index of the Merkle tree.
    ///
    /// # Examples
    ///
    /// Push the value 7 to the Merkle tree and get the index pushed to:
    ///
    /// ```rs
    /// let index = tree.push(7.into()).await?;
    /// ```
    ///
    pub async fn push(&mut self, value: Field) -> Result<u64> {
        let padding = {
            let file = &mut self.files[0];
            file.metadata().await?.len()
        };

        let index = padding / 32;

        self.set(index, value).await?;

        Ok(index)
    }

    /// Returns the witness of the leaf at the given index of the Merkle tree.
    ///
    /// # Examples
    ///
    /// Get the witness of the leaf at the 7th index:
    ///
    /// ```rs
    /// let witness_x1 = tree.get_witness_x1(7).await?;
    /// ```
    ///
    /// Calculate the root:
    ///
    /// ```rs
    /// let value = Field::from(42);
    /// let root = witness_x1.calculate_root(&value);
    /// ```
    ///
    pub async fn get_witness_x1(&mut self, mut index: u64) -> Result<WitnessX1<S, A>> {
        let mut siblings: [Sibling; S] = [Sibling::default(); S];

        if index > Self::MAX_INDEX {
            return Err(MerkleTreeError::NonExistentIndex(Self::MAX_INDEX));
        }

        for i in 0..S {
            let file = &mut self.files[i];
            let file_len = file.metadata().await?.len();

            let sibling_index = pick!(index % 2 == 0, index + 1, index - 1);

            let padding = sibling_index * 32;

            let sibling = if file_len == 0 || padding > file_len - 32 {
                self.zeroes[i]
            } else {
                let mut buf = [0_u8; 32];
                file.seek(SeekFrom::Start(padding)).await?;
                file.read_exact(&mut buf).await?;
                field_from_bytes(&buf)
            };

            siblings[i] = Sibling {
                value: sibling,
                is_left: sibling_index % 2 == 0,
            };

            index /= 2;
        }

        Ok(WitnessX1 { siblings })
    }

    /// Returns the witness of two leaves at the given indexes of the Merkle tree.
    ///
    /// # Examples
    ///
    /// Get the witness of the leaves at the 7th and 9th indexes:
    ///
    /// ```rs
    /// let witness_x2 = tree.get_witness_x2(7, 9).await?;
    /// ```
    ///
    /// Calculate the root:
    ///
    /// ```rs
    /// let value_x1 = Field::from(42);
    /// let value_x2 = Field::from(55);
    /// let root = witness_x2.calculate_root(&value_x1, &value_x2);
    /// ```
    ///
    pub async fn get_witness_x2(
        &mut self,
        mut index_x1: u64,
        mut index_x2: u64,
    ) -> Result<WitnessX2<S, B>> {
        let mut siblings_by_leaves: [[Sibling; S]; 2] =
            [[Sibling::default(); S], [Sibling::default(); S]];
        let mut siblings_at: [bool; S] = [false; S];
        let mut siblings_found = false;

        if index_x1 > Self::MAX_INDEX || index_x2 > Self::MAX_INDEX {
            return Err(MerkleTreeError::NonExistentIndex(Self::MAX_INDEX));
        }

        for i in 0..S {
            let file = &mut self.files[i];
            let file_len = file.metadata().await?.len();

            let sibling_index_x1 = pick!(index_x1 % 2 == 0, index_x1 + 1, index_x1 - 1);
            let sibling_index_x2 = pick!(index_x2 % 2 == 0, index_x2 + 1, index_x2 - 1);

            if sibling_index_x1 == index_x2 && !siblings_found {
                siblings_found = true;
                siblings_at[i] = true
            }

            let padding_x1 = sibling_index_x1 * 32;
            let padding_x2 = sibling_index_x2 * 32;

            let sibling_x1 = if file_len == 0 || padding_x1 > file_len - 32 {
                self.zeroes[i]
            } else {
                let mut buf = [0_u8; 32];
                file.seek(SeekFrom::Start(padding_x1)).await?;
                file.read_exact(&mut buf).await?;
                field_from_bytes(&buf)
            };

            let sibling_x2 = if file_len == 0 || padding_x2 > file_len - 32 {
                self.zeroes[i]
            } else {
                let mut buf = [0_u8; 32];
                file.seek(SeekFrom::Start(padding_x2)).await?;
                file.read_exact(&mut buf).await?;
                field_from_bytes(&buf)
            };

            siblings_by_leaves[0][i] = Sibling {
                value: sibling_x1,
                is_left: sibling_index_x1 % 2 == 0,
            };
            siblings_by_leaves[1][i] = Sibling {
                value: sibling_x2,
                is_left: sibling_index_x2 % 2 == 0,
            };

            index_x1 /= 2;
            index_x2 /= 2;
        }

        Ok(WitnessX2 {
            siblings_by_leaves,
            siblings_at,
        })
    }

    /// Returns the witness of three leaves at the given indexes of the Merkle tree.
    ///
    /// It requires that 2nd and 3rd leaves to be siblings at an earlier height than 1st leave.
    ///
    /// # Examples
    ///
    /// Get the witness of the leaves at the 7th, 13th, 15th indexes:
    ///
    /// ```rs
    /// let witness_x3 = tree.get_witness_x3(7, 13, 15).await?;
    /// ```
    ///
    /// Calculate the root:
    ///
    /// ```rs
    /// let value_x1 = Field::from(42);
    /// let value_x2 = Field::from(85);
    /// let value_x3 = Field::from(96);
    /// let root = witness_x3.calculate_root(&value_x1, &value_x2, &value_x3);
    /// ```
    ///
    pub async fn get_witness_x3(
        &mut self,
        mut index_x1: u64,
        mut index_x2: u64,
        mut index_x3: u64,
    ) -> Result<WitnessX3<S, C>> {
        let mut siblings_by_leaves: [[Sibling; S]; 3] = [
            [Sibling::default(); S],
            [Sibling::default(); S],
            [Sibling::default(); S],
        ];
        let mut siblings_at_by_leaves: [[bool; S]; 2] = [[false; S], [false; S]];
        let mut siblings_found_height_x1_x2_x3 = S;
        let mut siblings_found_height_x2_x3 = S;

        if index_x1 > Self::MAX_INDEX || index_x2 > Self::MAX_INDEX || index_x3 > Self::MAX_INDEX {
            return Err(MerkleTreeError::NonExistentIndex(Self::MAX_INDEX));
        }

        for i in 0..S {
            let file = &mut self.files[i];
            let file_len = file.metadata().await?.len();

            let sibling_index_x1 = pick!(index_x1 % 2 == 0, index_x1 + 1, index_x1 - 1);
            let sibling_index_x2 = pick!(index_x2 % 2 == 0, index_x2 + 1, index_x2 - 1);
            let sibling_index_x3 = pick!(index_x3 % 2 == 0, index_x3 + 1, index_x3 - 1);

            if sibling_index_x1 == index_x2 && siblings_found_height_x1_x2_x3 == S {
                siblings_found_height_x1_x2_x3 = i;
                siblings_at_by_leaves[0][i] = true
            }

            if sibling_index_x2 == index_x3 && siblings_found_height_x2_x3 == S {
                siblings_found_height_x2_x3 = i;
                siblings_at_by_leaves[1][i] = true
            }

            let padding_x1 = sibling_index_x1 * 32;
            let padding_x2 = sibling_index_x2 * 32;
            let padding_x3 = sibling_index_x3 * 32;

            let sibling_x1 = if file_len == 0 || padding_x1 > file_len - 32 {
                self.zeroes[i]
            } else {
                let mut buf = [0_u8; 32];
                file.seek(SeekFrom::Start(padding_x1)).await?;
                file.read_exact(&mut buf).await?;
                field_from_bytes(&buf)
            };

            let sibling_x2 = if file_len == 0 || padding_x2 > file_len - 32 {
                self.zeroes[i]
            } else {
                let mut buf = [0_u8; 32];
                file.seek(SeekFrom::Start(padding_x2)).await?;
                file.read_exact(&mut buf).await?;
                field_from_bytes(&buf)
            };

            let sibling_x3 = if file_len == 0 || padding_x3 > file_len - 32 {
                self.zeroes[i]
            } else {
                let mut buf = [0_u8; 32];
                file.seek(SeekFrom::Start(padding_x3)).await?;
                file.read_exact(&mut buf).await?;
                field_from_bytes(&buf)
            };

            siblings_by_leaves[0][i] = Sibling {
                value: sibling_x1,
                is_left: sibling_index_x1 % 2 == 0,
            };
            siblings_by_leaves[1][i] = Sibling {
                value: sibling_x2,
                is_left: sibling_index_x2 % 2 == 0,
            };
            siblings_by_leaves[2][i] = Sibling {
                value: sibling_x3,
                is_left: sibling_index_x3 % 2 == 0,
            };

            index_x1 /= 2;
            index_x2 /= 2;
            index_x3 /= 2;
        }

        if siblings_found_height_x1_x2_x3 > siblings_found_height_x2_x3 {
            Err(MerkleTreeError::MistakenOrderX3)
        } else {
            Ok(WitnessX3 {
                siblings_by_leaves,
                siblings_at_by_leaves,
            })
        }
    }

    /// Returns the witness of four leaves at the given indexes of the Merkle tree.
    ///
    /// It requires that 1st and 2nd leaves to be siblings at an earlier height than other leaves, the same is also valid for 3rd and 4th leaves.
    ///
    /// # Examples
    ///
    /// Get the witness of the leaves at the 7th, 8th, 17th, 20th indexes:
    ///
    /// ```rs
    /// let witness_x4 = tree.get_witness_x3(7, 8, 17, 20).await?;
    /// ```
    ///
    /// Calculate the root:
    ///
    /// ```rs
    /// let value_x1 = Field::from(42);
    /// let value_x2 = Field::from(57);
    /// let value_x3 = Field::from(63);
    /// let value_x4 = Field::from(78);
    /// let root = witness_x4.calculate_root(&value_x1, &value_x2, &value_x3, &value_x4);
    /// ```
    ///
    pub async fn get_witness_x4(
        &mut self,
        mut index_x1: u64,
        mut index_x2: u64,
        mut index_x3: u64,
        mut index_x4: u64,
    ) -> Result<WitnessX4<S, D>> {
        let mut siblings_by_leaves: [[Sibling; S]; 4] = [
            [Sibling::default(); S],
            [Sibling::default(); S],
            [Sibling::default(); S],
            [Sibling::default(); S],
        ];
        let mut siblings_at_by_leaves: [[bool; S]; 3] = [[false; S], [false; S], [false; S]];
        let mut siblings_found_height_x1_x2_x3_x4 = S;
        let mut siblings_found_height_x1_x2 = S;
        let mut siblings_found_height_x3_x4 = S;

        if index_x1 > Self::MAX_INDEX
            || index_x2 > Self::MAX_INDEX
            || index_x3 > Self::MAX_INDEX
            || index_x4 > Self::MAX_INDEX
        {
            return Err(MerkleTreeError::NonExistentIndex(Self::MAX_INDEX));
        }

        for i in 0..S {
            let file = &mut self.files[i];
            let file_len = file.metadata().await?.len();

            let sibling_index_x1 = pick!(index_x1 % 2 == 0, index_x1 + 1, index_x1 - 1);
            let sibling_index_x2 = pick!(index_x2 % 2 == 0, index_x2 + 1, index_x2 - 1);
            let sibling_index_x3 = pick!(index_x3 % 2 == 0, index_x3 + 1, index_x3 - 1);
            let sibling_index_x4 = pick!(index_x4 % 2 == 0, index_x4 + 1, index_x4 - 1);

            if sibling_index_x1 == index_x2 && siblings_found_height_x1_x2 == S {
                siblings_found_height_x1_x2 = i;
                siblings_at_by_leaves[0][i] = true
            }

            if sibling_index_x3 == index_x4 && siblings_found_height_x3_x4 == S {
                siblings_found_height_x3_x4 = i;
                siblings_at_by_leaves[1][i] = true
            }

            if sibling_index_x1 == index_x3 && siblings_found_height_x1_x2_x3_x4 == S {
                siblings_found_height_x1_x2_x3_x4 = i;
                siblings_at_by_leaves[2][i] = true
            }

            let padding_x1 = sibling_index_x1 * 32;
            let padding_x2 = sibling_index_x2 * 32;
            let padding_x3 = sibling_index_x3 * 32;
            let padding_x4 = sibling_index_x4 * 32;

            let sibling_x1 = if file_len == 0 || padding_x1 > file_len - 32 {
                self.zeroes[i]
            } else {
                let mut buf = [0_u8; 32];
                file.seek(SeekFrom::Start(padding_x1)).await?;
                file.read_exact(&mut buf).await?;
                field_from_bytes(&buf)
            };

            let sibling_x2 = if file_len == 0 || padding_x2 > file_len - 32 {
                self.zeroes[i]
            } else {
                let mut buf = [0_u8; 32];
                file.seek(SeekFrom::Start(padding_x2)).await?;
                file.read_exact(&mut buf).await?;
                field_from_bytes(&buf)
            };

            let sibling_x3 = if file_len == 0 || padding_x3 > file_len - 32 {
                self.zeroes[i]
            } else {
                let mut buf = [0_u8; 32];
                file.seek(SeekFrom::Start(padding_x3)).await?;
                file.read_exact(&mut buf).await?;
                field_from_bytes(&buf)
            };

            let sibling_x4 = if file_len == 0 || padding_x4 > file_len - 32 {
                self.zeroes[i]
            } else {
                let mut buf = [0_u8; 32];
                file.seek(SeekFrom::Start(padding_x4)).await?;
                file.read_exact(&mut buf).await?;
                field_from_bytes(&buf)
            };

            siblings_by_leaves[0][i] = Sibling {
                value: sibling_x1,
                is_left: sibling_index_x1 % 2 == 0,
            };
            siblings_by_leaves[1][i] = Sibling {
                value: sibling_x2,
                is_left: sibling_index_x2 % 2 == 0,
            };
            siblings_by_leaves[2][i] = Sibling {
                value: sibling_x3,
                is_left: sibling_index_x3 % 2 == 0,
            };

            siblings_by_leaves[3][i] = Sibling {
                value: sibling_x4,
                is_left: sibling_index_x4 % 2 == 0,
            };

            index_x1 /= 2;
            index_x2 /= 2;
            index_x3 /= 2;
            index_x4 /= 2;
        }

        if siblings_found_height_x1_x2 > siblings_found_height_x1_x2_x3_x4
            || siblings_found_height_x3_x4 > siblings_found_height_x1_x2_x3_x4
        {
            Err(MerkleTreeError::MistakenOrderX4)
        } else {
            Ok(WitnessX4 {
                siblings_by_leaves,
                siblings_at_by_leaves,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs::remove_dir_all;

    #[tokio::test]
    async fn creates_merkle_tree() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/merkle_tree/creates_merkle_tree";

        let _ = MerkleTree::<2>::new(dir).await?;

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn sets_and_gets_values() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/merkle_tree/sets_and_gets_values";

        let mut tree = MerkleTree::<5>::new(dir).await?;

        assert_eq!(tree.get(0).await?, Field::from(0));

        tree.set(0, Field::from(42)).await?;

        assert_eq!(tree.get(0).await?, Field::from(42));

        tree.set(0, Field::from(7)).await?;

        assert_eq!(tree.get(0).await?, Field::from(7));

        tree.set(1, Field::from(5)).await?;

        assert_eq!(tree.get(1).await?, Field::from(5));

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn pushes_values() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/merkle_tree/pushes_values";
        let mut tree = MerkleTree::<4>::new(dir).await?;

        assert_eq!(tree.get(0).await?, Field::from(0));

        tree.push(Field::from(42)).await?;

        assert_eq!(tree.get(0).await?, Field::from(42));

        tree.push(Field::from(5)).await?;

        assert_eq!(tree.get(1).await?, Field::from(5));

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn updates_root() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/merkle_tree/updates_root";
        let mut tree = MerkleTree::<6>::new(dir).await?;

        assert_eq!(
            tree.root().await?,
            "16556836945641263257329399459944072214107361158323688202689648863681494824075"
                .parse()
                .unwrap(),
        );

        tree.set(0, Field::from(42)).await?;

        assert_eq!(
            tree.root().await?,
            "11286019890017551530834192343235674860486670774006350997684329661758571079256"
                .parse()
                .unwrap(),
        );

        tree.set(0, Field::from(41)).await?;

        assert_eq!(
            tree.root().await?,
            "11232547598950213937608367704554422658642238970817973524109980433706405318743"
                .parse()
                .unwrap(),
        );

        tree.set(1, Field::from(7)).await?;

        assert_eq!(
            tree.root().await?,
            "14390539994532440242922216804268685968096299699558773027910939265009811713542"
                .parse()
                .unwrap(),
        );

        tree.set(2, Field::from(19)).await?;

        assert_eq!(
            tree.root().await?,
            "14405807291535085173183291373527489643529878893823965854142657852765785728874"
                .parse()
                .unwrap(),
        );

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn doesnt_set_unusable_index() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/merkle_tree/doesnt_set_unusable_index";

        let mut tree = MerkleTree::<42>::new(dir).await?;

        let unusable_index = 1;

        match tree
            .set(unusable_index, "42".parse().unwrap())
            .await
            .unwrap_err()
        {
            MerkleTreeError::UnusableIndex {
                given_index,
                usable_index,
            } => {
                assert_eq!(given_index, unusable_index);
                assert_eq!(usable_index, 0);
            }
            _ => unreachable!(),
        }

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn doesnt_set_non_existent_index() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let dir = "/tmp/nacho/tests/merkle_tree/doesnt_set_non_existent_index";

        let mut tree = MerkleTree::<42>::new(dir).await?;

        let non_existent_index = 2_u64.pow(41);

        match tree
            .set(non_existent_index, "42".parse().unwrap())
            .await
            .unwrap_err()
        {
            MerkleTreeError::NonExistentIndex {
                given_index,
                highest_possible_index,
            } => {
                assert_eq!(given_index, non_existent_index);
                assert_eq!(highest_possible_index, 2_u64.pow(41) - 1);
            }
            _ => unreachable!(),
        }

        Ok(remove_dir_all(dir).await?)
    }

    #[tokio::test]
    async fn gets_witness() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/merkle_tree/gets_witness";
        let mut tree = MerkleTree::<5>::new(dir).await?;

        let witness = tree.witness(0).await?;

        assert_eq!(
            witness.0,
            [
                "0".parse().unwrap(),
                "0".parse().unwrap(),
                "21565680844461314807147611702860246336805372493508489110556896454939225549736"
                    .parse()
                    .unwrap(),
                "2447983280988565496525732146838829227220882878955914181821218085513143393976"
                    .parse()
                    .unwrap(),
                "544619463418997333856881110951498501703454628897449993518845662251180546746"
                    .parse()
                    .unwrap(),
            ]
        );

        tree.set(0, "42".parse().unwrap()).await?;

        let witness = tree.witness(1).await?;

        assert_eq!(
            witness.0,
            [
                "1".parse().unwrap(),
                "42".parse().unwrap(),
                "21565680844461314807147611702860246336805372493508489110556896454939225549736"
                    .parse()
                    .unwrap(),
                "2447983280988565496525732146838829227220882878955914181821218085513143393976"
                    .parse()
                    .unwrap(),
                "544619463418997333856881110951498501703454628897449993518845662251180546746"
                    .parse()
                    .unwrap(),
            ]
        );

        Ok(remove_dir_all(dir).await?)
    }
}
