use crate::{put_in_order, Sibling};
use data_structures::Field;
use poseidon_hash::{poseidon_hash, PoseidonHasher};

/// The type that represents a Merkle witness of a leaf in a Merkle tree.
///
/// The constant generic parameter `S` is used for number of the siblings and `B` is used for the total size of the witness in bytes.
///
/// The constant generic parameter `B` must always be equal to `S * 33` for correct deserialization.
pub struct WitnessX1<const S: usize, const B: usize> {
    pub(crate) siblings: [Sibling; S],
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
    /// Calculates the root of the Merkle tree the witness represents using the given leaf values.
    ///
    /// # Examples
    ///
    /// Calculate root:
    ///
    /// ```rs
    /// let mut hasher = create_poseidon_hasher();
    /// let value: Field = 4.into();
    /// let root = witness_x1.calculate_root(&mut hasher, &value);
    /// ```
    ///
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

#[cfg(test)]
mod test {
    use super::*;
    use poseidon_hash::create_poseidon_hasher;

    /// A helper function to calculate and assert Merkle roots.
    fn assert_root<const S: usize, const B: usize>(
        hasher: &mut PoseidonHasher,
        witness_x1: &WitnessX1<S, B>,
        value_x1: &str,
        expected_root: &str,
    ) {
        assert_eq!(
            witness_x1.calculate_root(hasher, &value_x1.parse().unwrap()),
            expected_root.parse().unwrap()
        )
    }

    /// A helper function to construct Merkle witnesses.
    fn construct_witness<const S: usize, const B: usize>(
        siblings: [(&str, bool); S],
    ) -> WitnessX1<S, B> {
        WitnessX1::<S, B> {
            siblings: siblings.map(|(value, is_left)| Sibling {
                value: value.parse().unwrap(),
                is_left,
            }),
        }
    }

    #[test]
    fn calculates_root_using_leaf_0() {
        let mut hasher = create_poseidon_hasher();
        let hasher = &mut hasher;

        let witness = construct_witness::<3, 99>([
            ("0", false),
            (
                "21565680844461314807147611702860246336805372493508489110556896454939225549736",
                false,
            ),
            (
                "2447983280988565496525732146838829227220882878955914181821218085513143393976",
                false,
            ),
        ]);

        assert_root(
            hasher,
            &witness,
            "0",
            "544619463418997333856881110951498501703454628897449993518845662251180546746",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "26037750495375185037709120032423834194858338411345696972103621428999844391359",
        );
    }

    #[test]
    fn calculates_root_using_leaf_1() {
        let mut hasher = create_poseidon_hasher();
        let hasher = &mut hasher;

        let witness = construct_witness::<3, 99>([
            ("0", true),
            (
                "21565680844461314807147611702860246336805372493508489110556896454939225549736",
                false,
            ),
            (
                "2447983280988565496525732146838829227220882878955914181821218085513143393976",
                false,
            ),
        ]);

        assert_root(
            hasher,
            &witness,
            "0",
            "544619463418997333856881110951498501703454628897449993518845662251180546746",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "18342597908075387589456793626121922477502149401438274513604608381480672033060",
        );
    }

    #[test]
    fn calculates_root_using_leaf_5() {
        let mut hasher = create_poseidon_hasher();
        let hasher = &mut hasher;

        let witness = construct_witness::<3, 99>([
            ("0", true),
            (
                "21565680844461314807147611702860246336805372493508489110556896454939225549736",
                false,
            ),
            (
                "2447983280988565496525732146838829227220882878955914181821218085513143393976",
                true,
            ),
        ]);

        assert_root(
            hasher,
            &witness,
            "0",
            "544619463418997333856881110951498501703454628897449993518845662251180546746",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "3542506720567635734203174342003745665767633877002456239070049071059012301575",
        );
    }

    #[test]
    fn calculates_root_using_leaf_7() {
        let mut hasher = create_poseidon_hasher();
        let hasher = &mut hasher;

        let witness = construct_witness::<3, 99>([
            ("0", true),
            (
                "21565680844461314807147611702860246336805372493508489110556896454939225549736",
                true,
            ),
            (
                "2447983280988565496525732146838829227220882878955914181821218085513143393976",
                true,
            ),
        ]);

        assert_root(
            hasher,
            &witness,
            "0",
            "544619463418997333856881110951498501703454628897449993518845662251180546746",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "16739068883562948461523574250431506502899418775051902135199693929679649226337",
        );
    }

    #[test]
    fn calculates_root_of_filled_tree_using_leaf_4() {
        let mut hasher = create_poseidon_hasher();
        let hasher = &mut hasher;

        let witness = construct_witness::<3, 99>([
            ("5", false),
            (
                "27167074477134344337665931013283393666269936283998991735938568095272340697049",
                false,
            ),
            (
                "16005515111740441298758592150706683626801247123391878973458494605157646914613",
                true,
            ),
        ]);

        assert_root(
            hasher,
            &witness,
            "4",
            "8641025559269106317706019819017000366771390294969462887734789030067175090110",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "15285408421354356207006754773733283082461363485191536335771421138797092317821",
        );
    }
}
