use mina_hasher::Fp;
use mina_poseidon::{
    constants::PlonkSpongeConstantsKimchi,
    pasta::fp_kimchi,
    poseidon::{ArithmeticSponge, Sponge},
};

type Field = Fp;

/// The type for Poseidon hasher.
///
/// # Examples
///
/// ```rs
/// let poseidon_hasher = create_poseidon_hasher();
/// ```
///
pub type PoseidonHasher = ArithmeticSponge<Field, PlonkSpongeConstantsKimchi>;

/// Creates a new Posedion hasher.
///
/// # Examples
///
/// ```rs
/// let poseidon_hasher = create_poseidon_hasher();
/// ```
///
pub fn create_poseidon_hasher() -> PoseidonHasher {
    PoseidonHasher::new(fp_kimchi::static_params())
}

/// Returns the hash of the input using the given Poseidon hasher.
///
/// # Examples
///
/// ```rs
/// let hash = poseidon_hash(&mut poseidon_hasher, &input);
/// ```
///
pub fn poseidon_hash(poseidon_hasher: &mut PoseidonHasher, input: &[Field]) -> Field {
    poseidon_hasher.reset();
    poseidon_hasher.absorb(input);
    poseidon_hasher.squeeze()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_of_empty_input() {
        let mut poseidon_hasher = create_poseidon_hasher();

        let input = &[];

        let hash = poseidon_hash(&mut poseidon_hasher, input);
        let expected_hash =
            "21565680844461314807147611702860246336805372493508489110556896454939225549736"
                .parse()
                .unwrap();

        assert_eq!(hash, expected_hash);
    }

    #[test]
    fn test_hash_of_custom_input() {
        let mut poseidon_hasher = create_poseidon_hasher();

        let field: Field = "4657651324657865143213749874615453498767487414568798746541"
            .parse()
            .unwrap();

        let input = &[field];

        let hash = poseidon_hash(&mut poseidon_hasher, input);

        let expected_hash =
            "25092575324369866045572513480078649028026752517408112307655864726841403911786"
                .parse()
                .unwrap();

        assert_eq!(hash, expected_hash);
    }
}
