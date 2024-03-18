use crate::macros::{choose, is_sibling, swap};
use ark_ff::BigInteger256;
use data_structures::Field;
use poseidon_hash::{poseidon_hash, PoseidonHasher};

/// A Merkle witness that is just an array of `Field` elements.
///
/// The constant generic parameter `H` represents the height of the Merkle tree that witness belongs to.
///
/// The first item in the array is the index of the leaf the witness belongs to.
/// All the other items in the array are the sibling elements.
///
/// # Examples
///
/// Get a witness from a `MerkleTree`:
//
/// ```rs
/// let witness = tree.witness(7).await?;
/// ```
///
/// Construct a new witness using the given items:
///
/// ```rs
/// let witness = Witness::<42>::new(items);
/// ```
///
pub struct Witness<const H: usize>(pub(crate) [Field; H]);

impl<const H: usize> Witness<H> {
    /// Creates a new `Witness` using the given array of `Field` elements.
    ///
    /// # Examples
    ///
    /// Construct a new witness using the given items:
    ///
    /// ```rs
    /// let witness = Witness::<42>::new(items);
    /// ```
    ///
    pub fn new(items: [Field; H]) -> Self {
        Self(items)
    }

    /// Calculates a Merkle root using one Merkle witness and one value.
    ///
    /// # Examples
    ///
    /// Calculate a root:
    /// ```rs
    /// let root = Witness::calculate_root_x1(&mut hasher, witness, value);
    /// ```
    ///
    pub fn calculate_root_x1(
        hasher: &mut PoseidonHasher,
        witness_x1: Witness<H>,
        value_x1: Field,
    ) -> Field {
        let bigint_x1: BigInteger256 = witness_x1.0[0].into();
        let mut index_x1 = bigint_x1.0[0];

        let mut root_x1 = value_x1;

        for i in 1..H {
            let sibling_x1 = witness_x1.0[i];

            let is_left_x1 = index_x1 % 2 == 0;

            let (l_x1, r_x1) = swap!(is_left_x1, sibling_x1, root_x1);
            root_x1 = poseidon_hash(hasher, &[l_x1, r_x1]);

            index_x1 = index_x1 / 2;
        }

        root_x1
    }

    /// Calculates a Merkle root using two Merkle witnesses and two values.
    ///
    /// # Examples
    ///
    /// Calculate a root:
    /// ```rs
    /// let root = Witness::calculate_root_x2(
    ///     &mut hasher,
    ///     witness_x1,
    ///     value_x1,
    ///     witness_x2,
    ///     value_x2
    /// );
    /// ```
    ///
    pub fn calculate_root_x2(
        hasher: &mut PoseidonHasher,
        witness_x1: Witness<H>,
        value_x1: Field,
        witness_x2: Witness<H>,
        value_x2: Field,
    ) -> Field {
        let bigint_x1: BigInteger256 = witness_x1.0[0].into();
        let bigint_x2: BigInteger256 = witness_x2.0[0].into();
        let mut index_x1 = bigint_x1.0[0];
        let mut index_x2 = bigint_x2.0[0];

        let mut root_x1 = value_x1;
        let mut root_x2 = value_x2;

        for i in 1..H {
            let sibling_x1 = choose!(is_sibling!(index_x1, index_x2), root_x2, witness_x1.0[i]);
            let sibling_x2 = witness_x2.0[i];

            let is_left_x1 = index_x1 % 2 == 0;
            let is_left_x2 = index_x2 % 2 == 0;

            let (l_x1, r_x1) = swap!(is_left_x1, sibling_x1, root_x1);
            root_x1 = poseidon_hash(hasher, &[l_x1, r_x1]);

            let (l_x2, r_x2) = swap!(is_left_x2, sibling_x2, root_x2);
            root_x2 = poseidon_hash(hasher, &[l_x2, r_x2]);

            index_x1 = index_x1 / 2;
            index_x2 = index_x2 / 2;
        }

        root_x1
    }
}

#[cfg(test)]
mod tests {
    use poseidon_hash::create_poseidon_hasher;

    use super::*;

    #[test]
    fn calculates_correct_roots_using_one_witness() {
        let mut hasher = create_poseidon_hasher();

        let items = [
            "0".parse().unwrap(),
            "1".parse().unwrap(),
            "21668148948031173567586057521910089298565671981886790533984285929362428405478"
                .parse()
                .unwrap(),
            "28698269317824367043440705550036962358277758892564867065835110044337335936454"
                .parse()
                .unwrap(),
        ];

        let witness = Witness::new(items);

        let value: Field = 0.into();

        let root = Witness::calculate_root_x1(&mut hasher, witness, value);

        assert_eq!(
            root,
            "8641025559269106317706019819017000366771390294969462887734789030067175090110"
                .parse()
                .unwrap()
        );

        let items = [
            "1".parse().unwrap(),
            "0".parse().unwrap(),
            "21668148948031173567586057521910089298565671981886790533984285929362428405478"
                .parse()
                .unwrap(),
            "28698269317824367043440705550036962358277758892564867065835110044337335936454"
                .parse()
                .unwrap(),
        ];

        let witness = Witness::new(items);

        let value: Field = 1.into();

        let root = Witness::calculate_root_x1(&mut hasher, witness, value);

        assert_eq!(
            root,
            "8641025559269106317706019819017000366771390294969462887734789030067175090110"
                .parse()
                .unwrap()
        );

        let items = [
            "2".parse().unwrap(),
            "3".parse().unwrap(),
            "25153834528238352025091411039949114579843839670440790727153524232958326376354"
                .parse()
                .unwrap(),
            "28698269317824367043440705550036962358277758892564867065835110044337335936454"
                .parse()
                .unwrap(),
        ];

        let witness = Witness::new(items);

        let value: Field = 2.into();

        let root = Witness::calculate_root_x1(&mut hasher, witness, value);

        assert_eq!(
            root,
            "8641025559269106317706019819017000366771390294969462887734789030067175090110"
                .parse()
                .unwrap()
        );

        let items = [
            "5".parse().unwrap(),
            "4".parse().unwrap(),
            "27167074477134344337665931013283393666269936283998991735938568095272340697049"
                .parse()
                .unwrap(),
            "16005515111740441298758592150706683626801247123391878973458494605157646914613"
                .parse()
                .unwrap(),
        ];

        let witness = Witness::new(items);

        let value: Field = 5.into();

        let root = Witness::calculate_root_x1(&mut hasher, witness, value);

        assert_eq!(
            root,
            "8641025559269106317706019819017000366771390294969462887734789030067175090110"
                .parse()
                .unwrap()
        );
    }

    #[test]
    fn calculates_correct_roots_using_two_witnesses() {
        let mut hasher = create_poseidon_hasher();

        let items_x1 = [
            "0".parse().unwrap(),
            "1".parse().unwrap(),
            "21668148948031173567586057521910089298565671981886790533984285929362428405478"
                .parse()
                .unwrap(),
            "28698269317824367043440705550036962358277758892564867065835110044337335936454"
                .parse()
                .unwrap(),
        ];

        let items_x2 = [
            "1".parse().unwrap(),
            "0".parse().unwrap(),
            "21668148948031173567586057521910089298565671981886790533984285929362428405478"
                .parse()
                .unwrap(),
            "28698269317824367043440705550036962358277758892564867065835110044337335936454"
                .parse()
                .unwrap(),
        ];

        let witness_x1 = Witness::new(items_x1);
        let witness_x2 = Witness::new(items_x2);

        let value_x1: Field = 0.into();
        let value_x2: Field = 1.into();

        let root =
            Witness::calculate_root_x2(&mut hasher, witness_x1, value_x1, witness_x2, value_x2);

        assert_eq!(
            root,
            "8641025559269106317706019819017000366771390294969462887734789030067175090110"
                .parse()
                .unwrap()
        );

        let items_x1 = [
            "2".parse().unwrap(),
            "3".parse().unwrap(),
            "25153834528238352025091411039949114579843839670440790727153524232958326376354"
                .parse()
                .unwrap(),
            "28698269317824367043440705550036962358277758892564867065835110044337335936454"
                .parse()
                .unwrap(),
        ];

        let items_x2 = [
            "5".parse().unwrap(),
            "4".parse().unwrap(),
            "27167074477134344337665931013283393666269936283998991735938568095272340697049"
                .parse()
                .unwrap(),
            "16005515111740441298758592150706683626801247123391878973458494605157646914613"
                .parse()
                .unwrap(),
        ];

        let witness_x1 = Witness::new(items_x1);
        let witness_x2 = Witness::new(items_x2);

        let value_x1: Field = 2.into();
        let value_x2: Field = 5.into();

        let root =
            Witness::calculate_root_x2(&mut hasher, witness_x1, value_x1, witness_x2, value_x2);

        assert_eq!(
            root,
            "8641025559269106317706019819017000366771390294969462887734789030067175090110"
                .parse()
                .unwrap()
        );
    }
}
