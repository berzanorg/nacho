use ark_ff::PrimeField;
use mina_signer::ScalarField;

use crate::Field;

/// An alias that represents a scalar field element.
pub(crate) type Scalar = ScalarField;

/// Converts the given field element into a scalar.
///
/// # Examples
///
/// ```rs
/// let scalar = field_to_scalar(42.into());
/// ```
pub(crate) fn field_to_scalar(field: Field) -> Scalar {
    field.into_repr().into()
}

/// Converts the given scalar into a field element.
///
/// # Examples
///
/// ```rs
/// let field = scalar_to_field(42.into());
/// ```
pub(crate) fn scalar_to_field(scalar: Scalar) -> Field {
    scalar.into_repr().into()
}
