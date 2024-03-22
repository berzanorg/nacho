use ark_ff::{PrimeField, SquareRootField};

use crate::{errors::GroupError, Field, PublicKey};

pub struct Group {
    pub x: Field,
    pub y: Field,
}

impl TryFrom<&PublicKey> for Group {
    type Error = GroupError;

    fn try_from(value: &PublicKey) -> Result<Self, Self::Error> {
        let y_squared = value.x * value.x + Field::from(5);

        let some_y = y_squared
            .sqrt()
            .ok_or(GroupError::CannotCreateFromPublicKey)?;
        let some_y_first_bit = some_y.into_repr().0[0].to_le_bytes()[0];

        let is_the_right_y = value.is_odd == (some_y_first_bit != 0);
        let is_the_right_y_not: Field = (!is_the_right_y).into();
        let is_the_right_y: Field = is_the_right_y.into();

        let y = is_the_right_y * some_y + is_the_right_y_not * (some_y * Field::from(-1));

        Ok(Self { x: value.x, y })
    }
}
