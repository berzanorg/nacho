use crate::{ByteConversion, Field, FieldConversion, Sibling, SingleMerkleWitness};
use nacho_macros::{choose, put_in_order};
use nacho_poseidon_hash::{poseidon_hash, PoseidonHasher};

/// The witness of a two leaves in a Merkle tree.
///
/// The constant generic `L` represents the number of siblings needed for a witness in a Merkle tree which is `tree_height - 1`.
///
/// A `DoubleMerkleWitness` is not constructed manually but constructed by Merkle trees.
#[derive(Clone, Debug)]
pub struct DoubleMerkleWitness<const L: usize> {
    /// Siblings of the first leaf the witness belongs to.
    pub siblings_x1: [Sibling; L],
    /// Siblings of the second leaf the witness belongs to.
    pub siblings_x2: [Sibling; L],
    /// The array where every item is `false` except the one where both leaves are each other's siblings.
    pub siblings_at: [bool; L],
}

impl<const L: usize> DoubleMerkleWitness<L> {
    /// Calculates a Merkle tree root using the given value.
    ///
    /// # Examples
    ///
    /// Calculate a root:
    ///
    /// ```rs
    /// let root = witness.calculate_root(&mut hasher, &value_x1, &value_x2);
    /// ```
    ///
    pub fn calculate_root(
        &self,
        hasher: &mut PoseidonHasher,
        value_x1: &Field,
        value_x2: &Field,
    ) -> Field {
        let mut root_x1 = value_x1.clone();
        let mut root_x2 = value_x2.clone();

        for i in 0..L {
            let sibling_x1 = choose!(
                self.siblings_at[i],
                root_x2,
                self.siblings_x1[i].value.to_fields()[0]
            );

            root_x1 = poseidon_hash(
                hasher,
                put_in_order!(self.siblings_x1[i].is_left, &[root_x1, sibling_x1]),
            );

            root_x2 = poseidon_hash(
                hasher,
                put_in_order!(
                    self.siblings_x2[i].is_left,
                    &[root_x2, self.siblings_x2[i].value.to_fields()[0]]
                ),
            );
        }

        root_x1
    }
}

/// The macro that implements `ByteConversion` trait for the double witness of the Merkle tree with the given height.
///
/// # Usage
///
/// Implement `ByteConversion` trait:
///
/// ```rs
/// impl_byte_conversion_trait_for_height!(23);
/// ```
///
macro_rules! impl_byte_conversion_trait_for_height {
    ($tree_height:literal) => {
        impl ByteConversion<{ 67 * ($tree_height - 1) }>
            for DoubleMerkleWitness<{ $tree_height - 1 }>
        {
            fn to_bytes(&self) -> [u8; { 67 * ($tree_height - 1) }] {
                let mut buf = [0u8; { 67 * ($tree_height - 1) }];

                for (i, sibling) in self.siblings_x1.iter().enumerate() {
                    let padding = i * 33;
                    buf[padding..padding + 32].copy_from_slice(&sibling.value.to_bytes());
                    buf[padding + 32] = sibling.is_left as u8;
                }

                for (i, sibling) in self.siblings_x2.iter().enumerate() {
                    let padding = ({ $tree_height - 1 } * 33) + (i * 33);
                    buf[padding..padding + 32].copy_from_slice(&sibling.value.to_bytes());
                    buf[padding + 32] = sibling.is_left as u8;
                }

                for (i, &siblings_at) in self.siblings_at.iter().enumerate() {
                    let padding = (2 * { $tree_height - 1 } * 33) + (i);

                    buf[padding] = siblings_at as u8;
                }

                buf
            }

            fn from_bytes(_: &[u8; { 67 * ($tree_height - 1) }]) -> Self {
                panic!("this function is not intended for use")
            }
        }
    };
}

impl_byte_conversion_trait_for_height!(23);

impl<const L: usize> From<(SingleMerkleWitness<L>, SingleMerkleWitness<L>)>
    for DoubleMerkleWitness<L>
{
    fn from(value: (SingleMerkleWitness<L>, SingleMerkleWitness<L>)) -> Self {
        let mut siblings_at = [false; L];

        for i in 0..L {
            if value.0.siblings[L - i - 1].is_left != value.1.siblings[L - i - 1].is_left {
                siblings_at[L - i - 1] = true;
                break;
            }
        }

        Self {
            siblings_x1: value.0.siblings,
            siblings_x2: value.1.siblings,
            siblings_at,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use nacho_poseidon_hash::create_poseidon_hasher;
    use std::array;

    /// A helper function to calculate and assert Merkle roots.
    fn assert_root<const L: usize>(
        hasher: &mut PoseidonHasher,
        double_witness: &DoubleMerkleWitness<L>,
        value_x1: &str,
        value_x2: &str,
        expected_root: &str,
    ) {
        assert_eq!(
            double_witness.calculate_root(
                hasher,
                &value_x1.parse().unwrap(),
                &value_x2.parse().unwrap()
            ),
            expected_root.parse().unwrap()
        )
    }

    /// A helper function to construct Merkle witnesses.
    fn construct_witness<const L: usize>(
        siblings_x1: [(&str, bool); L],
        siblings_x2: [(&str, bool); L],
    ) -> DoubleMerkleWitness<L> {
        let mut siblings_at_leaf = L;

        for i in 0..L {
            let i = L - 1 - i;

            if siblings_x1[i].1 == !siblings_x2[i].1 {
                siblings_at_leaf = i;
                break;
            }
        }

        DoubleMerkleWitness::<L> {
            siblings_x1: siblings_x1.map(|(value, is_left)| Sibling {
                value: value.parse::<Field>().unwrap().into(),
                is_left,
            }),
            siblings_x2: siblings_x2.map(|(value, is_left)| Sibling {
                value: value.parse::<Field>().unwrap().into(),
                is_left,
            }),
            siblings_at: array::from_fn(|i| if i == siblings_at_leaf { true } else { false }),
        }
    }

    #[test]
    fn calculates_root_using_leaves_0_and_1() {
        let mut hasher = create_poseidon_hasher();
        let hasher = &mut hasher;

        let witness = construct_witness::<3>(
            [
                ("0", false),
                (
                    "21565680844461314807147611702860246336805372493508489110556896454939225549736",
                    false,
                ),
                (
                    "2447983280988565496525732146838829227220882878955914181821218085513143393976",
                    false,
                ),
            ],
            [
                ("0", true),
                (
                    "21565680844461314807147611702860246336805372493508489110556896454939225549736",
                    false,
                ),
                (
                    "2447983280988565496525732146838829227220882878955914181821218085513143393976",
                    false,
                ),
            ],
        );

        assert_root(
            hasher,
            &witness,
            "0",
            "0",
            "544619463418997333856881110951498501703454628897449993518845662251180546746",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "0",
            "26037750495375185037709120032423834194858338411345696972103621428999844391359",
        );

        assert_root(
            hasher,
            &witness,
            "0",
            "123456798987654321",
            "18342597908075387589456793626121922477502149401438274513604608381480672033060",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "123456798987654321",
            "12884227973391712663688202009500842443230250394416820972072286191579209224052",
        );
    }

    #[test]
    fn calculates_root_using_leaves_3_and_4() {
        let mut hasher = create_poseidon_hasher();
        let hasher = &mut hasher;

        let witness = construct_witness::<3>(
            [
                ("0", true),
                (
                    "21565680844461314807147611702860246336805372493508489110556896454939225549736",
                    true,
                ),
                (
                    "2447983280988565496525732146838829227220882878955914181821218085513143393976",
                    false,
                ),
            ],
            [
                ("0", false),
                (
                    "21565680844461314807147611702860246336805372493508489110556896454939225549736",
                    false,
                ),
                (
                    "2447983280988565496525732146838829227220882878955914181821218085513143393976",
                    true,
                ),
            ],
        );

        assert_root(
            hasher,
            &witness,
            "0",
            "0",
            "544619463418997333856881110951498501703454628897449993518845662251180546746",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "0",
            "8186495563377223226891457428900067727518605824081403624305417583031097720957",
        );

        assert_root(
            hasher,
            &witness,
            "0",
            "123456798987654321",
            "8636238497261353849496583396674123542951288152062560417724244896103032519178",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "123456798987654321",
            "22148337222181318285458252657999718370945392512489257984465328423086124311833",
        );
    }

    #[test]
    fn calculates_root_using_leaves_2_and_5() {
        let mut hasher = create_poseidon_hasher();
        let hasher = &mut hasher;

        let witness = construct_witness::<3>(
            [
                ("0", false),
                (
                    "21565680844461314807147611702860246336805372493508489110556896454939225549736",
                    true,
                ),
                (
                    "2447983280988565496525732146838829227220882878955914181821218085513143393976",
                    false,
                ),
            ],
            [
                ("0", true),
                (
                    "21565680844461314807147611702860246336805372493508489110556896454939225549736",
                    false,
                ),
                (
                    "2447983280988565496525732146838829227220882878955914181821218085513143393976",
                    true,
                ),
            ],
        );

        assert_root(
            hasher,
            &witness,
            "0",
            "0",
            "544619463418997333856881110951498501703454628897449993518845662251180546746",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "0",
            "23590435126008625138258823899208789512021327812820307886722062151747051137010",
        );

        assert_root(
            hasher,
            &witness,
            "0",
            "123456798987654321",
            "3542506720567635734203174342003745665767633877002456239070049071059012301575",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "123456798987654321",
            "8876360771458208145127320833726189972718448447168011204921046844770481470637",
        );
    }

    #[test]
    fn calculates_root_using_leaves_6_and_7() {
        let mut hasher = create_poseidon_hasher();
        let hasher = &mut hasher;

        let witness = construct_witness::<3>(
            [
                ("0", false),
                (
                    "21565680844461314807147611702860246336805372493508489110556896454939225549736",
                    true,
                ),
                (
                    "2447983280988565496525732146838829227220882878955914181821218085513143393976",
                    true,
                ),
            ],
            [
                ("0", true),
                (
                    "21565680844461314807147611702860246336805372493508489110556896454939225549736",
                    true,
                ),
                (
                    "2447983280988565496525732146838829227220882878955914181821218085513143393976",
                    true,
                ),
            ],
        );

        assert_root(
            hasher,
            &witness,
            "0",
            "0",
            "544619463418997333856881110951498501703454628897449993518845662251180546746",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "0",
            "17008988363900846452827788243294622498145144109912296901638129532463861369215",
        );

        assert_root(
            hasher,
            &witness,
            "0",
            "123456798987654321",
            "16739068883562948461523574250431506502899418775051902135199693929679649226337",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "123456798987654321",
            "25152901830966197351673261482084109302331234457972214507797177749275523599230",
        );
    }

    #[test]
    fn calculates_root_using_leaves_6_and_4() {
        let mut hasher = create_poseidon_hasher();
        let hasher = &mut hasher;

        let witness = construct_witness::<3>(
            [
                ("0", false),
                (
                    "21565680844461314807147611702860246336805372493508489110556896454939225549736",
                    true,
                ),
                (
                    "2447983280988565496525732146838829227220882878955914181821218085513143393976",
                    true,
                ),
            ],
            [
                ("0", false),
                (
                    "21565680844461314807147611702860246336805372493508489110556896454939225549736",
                    false,
                ),
                (
                    "2447983280988565496525732146838829227220882878955914181821218085513143393976",
                    true,
                ),
            ],
        );

        assert_root(
            hasher,
            &witness,
            "0",
            "0",
            "544619463418997333856881110951498501703454628897449993518845662251180546746",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "0",
            "17008988363900846452827788243294622498145144109912296901638129532463861369215",
        );

        assert_root(
            hasher,
            &witness,
            "0",
            "123456798987654321",
            "8636238497261353849496583396674123542951288152062560417724244896103032519178",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "123456798987654321",
            "6979258273212218775590645562624760169275275548100902464926093354084226683405",
        );
    }

    #[test]
    fn calculates_root_using_leaves_0_and_7() {
        let mut hasher = create_poseidon_hasher();
        let hasher = &mut hasher;

        let witness = construct_witness::<3>(
            [
                ("0", false),
                (
                    "21565680844461314807147611702860246336805372493508489110556896454939225549736",
                    false,
                ),
                (
                    "2447983280988565496525732146838829227220882878955914181821218085513143393976",
                    false,
                ),
            ],
            [
                ("0", true),
                (
                    "21565680844461314807147611702860246336805372493508489110556896454939225549736",
                    true,
                ),
                (
                    "2447983280988565496525732146838829227220882878955914181821218085513143393976",
                    true,
                ),
            ],
        );

        assert_root(
            hasher,
            &witness,
            "0",
            "0",
            "544619463418997333856881110951498501703454628897449993518845662251180546746",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "0",
            "26037750495375185037709120032423834194858338411345696972103621428999844391359",
        );

        assert_root(
            hasher,
            &witness,
            "0",
            "123456798987654321",
            "16739068883562948461523574250431506502899418775051902135199693929679649226337",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "123456798987654321",
            "6975527369259688739140892142373169897245363181445047280715470848715312953601",
        );
    }

    #[test]
    fn calculates_root_of_filled_tree_using_leaves_3_and_6() {
        let mut hasher = create_poseidon_hasher();
        let hasher = &mut hasher;

        let witness = construct_witness::<3>(
            [
                ("2", true),
                (
                    "25153834528238352025091411039949114579843839670440790727153524232958326376354",
                    true,
                ),
                (
                    "28698269317824367043440705550036962358277758892564867065835110044337335936454",
                    false,
                ),
            ],
            [
                ("7", false),
                (
                    "15261178828524791341567814310696956704403579291352826057127000533880552733785",
                    true,
                ),
                (
                    "16005515111740441298758592150706683626801247123391878973458494605157646914613",
                    true,
                ),
            ],
        );

        assert_root(
            hasher,
            &witness,
            "3",
            "6",
            "8641025559269106317706019819017000366771390294969462887734789030067175090110",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "6",
            "719662910361950081888432835317304259246460886361937094675139594465855209323",
        );

        assert_root(
            hasher,
            &witness,
            "3",
            "123456798987654321",
            "11494114919731066013708293930913103068645643038203705306559040741103858060132",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "123456798987654321",
            "10994783466719201472089321785559140416227725009177562505936465154902000920644",
        );
    }

    #[test]
    fn calculates_root_using_same_leaf_5() {
        let mut hasher = create_poseidon_hasher();
        let hasher = &mut hasher;

        let witness = construct_witness::<3>(
            [
                ("0", true),
                (
                    "21565680844461314807147611702860246336805372493508489110556896454939225549736",
                    false,
                ),
                (
                    "2447983280988565496525732146838829227220882878955914181821218085513143393976",
                    true,
                ),
            ],
            [
                ("0", true),
                (
                    "21565680844461314807147611702860246336805372493508489110556896454939225549736",
                    false,
                ),
                (
                    "2447983280988565496525732146838829227220882878955914181821218085513143393976",
                    true,
                ),
            ],
        );

        assert_root(
            hasher,
            &witness,
            "0",
            "0",
            "544619463418997333856881110951498501703454628897449993518845662251180546746",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "0",
            "3542506720567635734203174342003745665767633877002456239070049071059012301575",
        );

        assert_root(
            hasher,
            &witness,
            "0",
            "123456798987654321",
            "544619463418997333856881110951498501703454628897449993518845662251180546746",
        );

        assert_root(
            hasher,
            &witness,
            "123456798987654321",
            "123456798987654321",
            "3542506720567635734203174342003745665767633877002456239070049071059012301575",
        );
    }
}
