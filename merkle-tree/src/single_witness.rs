use crate::{put_in_order, Sibling};
use nacho_data_structures::{Field, ToBytes};
use nacho_poseidon_hash::{poseidon_hash, PoseidonHasher};

pub struct SingleWitness<const L: usize> {
    pub siblings: [Sibling; L],
}

impl<const L: usize> SingleWitness<L> {
    pub fn calculate_root(&self, hasher: &mut PoseidonHasher, value: &Field) -> Field {
        let mut root = value.clone();

        for i in 0..L {
            root = poseidon_hash(
                hasher,
                put_in_order!(
                    self.siblings[i].is_left,
                    &[root, Field::from(&self.siblings[i].value)]
                ),
            );
        }

        root
    }
}

impl ToBytes for SingleWitness<22> {
    type Bytes = [u8; 726];

    fn to_bytes(&self) -> Self::Bytes {
        let mut buf = [0u8; 726];

        for (i, sibling) in self.siblings.iter().enumerate() {
            let padding = i * 33;
            buf[padding..padding + 32].copy_from_slice(&sibling.value.to_bytes());
            buf[padding + 32] = sibling.is_left as u8;
        }

        buf
    }
}

impl ToBytes for SingleWitness<21> {
    type Bytes = [u8; 693];

    fn to_bytes(&self) -> Self::Bytes {
        let mut buf = [0u8; 693];

        for (i, sibling) in self.siblings.iter().enumerate() {
            let padding = i * 33;
            buf[padding..padding + 32].copy_from_slice(&sibling.value.to_bytes());
            buf[padding + 32] = sibling.is_left as u8;
        }

        buf
    }
}

impl ToBytes for SingleWitness<20> {
    type Bytes = [u8; 660];

    fn to_bytes(&self) -> Self::Bytes {
        let mut buf = [0u8; 660];

        for (i, sibling) in self.siblings.iter().enumerate() {
            let padding = i * 33;
            buf[padding..padding + 32].copy_from_slice(&sibling.value.to_bytes());
            buf[padding + 32] = sibling.is_left as u8;
        }

        buf
    }
}

impl ToBytes for SingleWitness<19> {
    type Bytes = [u8; 627];

    fn to_bytes(&self) -> Self::Bytes {
        let mut buf = [0u8; 627];

        for (i, sibling) in self.siblings.iter().enumerate() {
            let padding = i * 33;
            buf[padding..padding + 32].copy_from_slice(&sibling.value.to_bytes());
            buf[padding + 32] = sibling.is_left as u8;
        }

        buf
    }
}

impl ToBytes for SingleWitness<18> {
    type Bytes = [u8; 594];

    fn to_bytes(&self) -> Self::Bytes {
        let mut buf = [0u8; 594];

        for (i, sibling) in self.siblings.iter().enumerate() {
            let padding = i * 33;
            buf[padding..padding + 32].copy_from_slice(&sibling.value.to_bytes());
            buf[padding + 32] = sibling.is_left as u8;
        }

        buf
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use nacho_poseidon_hash::create_poseidon_hasher;

    /// A helper function to calculate and assert Merkle roots.
    fn assert_root<const L: usize>(
        hasher: &mut PoseidonHasher,
        single_witness: &SingleWitness<L>,
        value: &str,
        expected_root: &str,
    ) {
        assert_eq!(
            single_witness.calculate_root(hasher, &value.parse().unwrap()),
            expected_root.parse().unwrap()
        )
    }

    /// A helper function to construct Merkle witnesses.
    fn construct_witness<const L: usize>(siblings: [(&str, bool); L]) -> SingleWitness<L> {
        SingleWitness::<L> {
            siblings: siblings.map(|(value, is_left)| Sibling {
                value: value.parse::<Field>().unwrap().into(),
                is_left,
            }),
        }
    }

    #[test]
    fn calculates_root_using_leaf_0() {
        let mut hasher = create_poseidon_hasher();
        let hasher = &mut hasher;

        let witness = construct_witness::<3>([
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

        let witness = construct_witness::<3>([
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

        let witness = construct_witness::<3>([
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

        let witness = construct_witness::<3>([
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

        let witness = construct_witness::<3>([
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
