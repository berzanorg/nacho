use crate::{Field, ToFields};
use std::ops::{Add, Div, Mul, Rem, Sub};
use thiserror::Error;

/// A wrapper around `Field` with bounds checking to act like a 64-bit unsigned integer.
///
/// # Examples
///
/// ```rs
/// let field: Field = 42.into();
/// let uint64 = Uint64::new(field)?;
///
/// let num7 = Uint64::new(7.into())?;
/// let num5 = Uint64::new(5.into())?;
///
/// let num12 = (num7 + num5)?;
/// let num2 = (num7 - num5)?;
/// let num35 = (num7 * num5)?;
/// let num1 = (num7 / num5)?;
/// let num2 = (num7 % num5)?;
/// ```
///
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Uint64 {
    pub value: Field,
}

impl ToFields<1> for Uint64 {
    fn to_fields(&self) -> [Field; 1] {
        [self.value]
    }
}

/// The error type that represents the errors that can happen during arithmetic operations on `Uint64`.
#[derive(Error, Debug)]
pub enum Uint64Error {
    #[error("Uint64 is out of range")]
    OutOfRange,
}

impl Uint64 {
    /// Creates a new `Uint64` using given the given `Field` value.
    ///
    /// Returns a `Uint64Error` if the given value is greater than 2^64.
    ///
    /// # Examples
    ///
    /// ```rs
    /// let field: Field = 42.into();
    /// let uint64 = Uint64::new(field)?;
    ///
    /// let uint64 = Uint64::new(7.into())?;
    /// ```
    ///
    pub fn new(value: Field) -> Result<Uint64, Uint64Error> {
        if Uint64::is_out_of_range(value) {
            Err(Uint64Error::OutOfRange)
        } else {
            Ok(Uint64 { value })
        }
    }

    /// Returns `true` if the given `Field` value is greater than 2^64.
    ///
    /// Otherwise, returns `false`.
    ///
    /// # Examples
    ///
    /// ```rs
    /// let field = 42.into();
    ///
    /// if Uint64::is_out_of_range(field) {
    ///     // return an error
    /// } else {
    ///     // continue
    /// }
    /// ```
    ///
    #[inline]
    pub fn is_out_of_range(value: Field) -> bool {
        return value > Field::from(u64::MAX);
    }
}

impl Add for Uint64 {
    type Output = Result<Uint64, Uint64Error>;

    fn add(self, rhs: Self) -> Self::Output {
        let field = self.value + rhs.value;

        if Uint64::is_out_of_range(field) {
            Err(Uint64Error::OutOfRange)
        } else {
            Ok(Uint64 { value: field })
        }
    }
}

impl Sub for Uint64 {
    type Output = Result<Uint64, Uint64Error>;

    fn sub(self, rhs: Self) -> Self::Output {
        let field = self.value - rhs.value;

        if Uint64::is_out_of_range(field) {
            Err(Uint64Error::OutOfRange)
        } else {
            Ok(Uint64 { value: field })
        }
    }
}

impl Mul for Uint64 {
    type Output = Result<Uint64, Uint64Error>;

    fn mul(self, rhs: Self) -> Self::Output {
        let field = self.value * rhs.value;

        if Uint64::is_out_of_range(field) {
            Err(Uint64Error::OutOfRange)
        } else {
            Ok(Uint64 { value: field })
        }
    }
}

impl Div for Uint64 {
    type Output = Result<Uint64, Uint64Error>;

    fn div(self, rhs: Self) -> Self::Output {
        // TODO: Fix the error that happens when `rhs` isn't a multiple of `self`.
        let field = self.value / rhs.value;

        if Uint64::is_out_of_range(field) {
            Err(Uint64Error::OutOfRange)
        } else {
            Ok(Uint64 { value: field })
        }
    }
}

impl Rem for Uint64 {
    type Output = Result<Uint64, Uint64Error>;

    fn rem(self, rhs: Self) -> Self::Output {
        let div = (self / rhs)?;

        let mul = (div * rhs)?;

        self - mul
    }
}

#[test]
fn adds_uint64() -> Result<(), Box<dyn std::error::Error>> {
    let x = Uint64::new(42.into())?;
    let y = Uint64::new(7.into())?;
    let z = (x + y)?;

    assert_eq!(z, Uint64::new(49.into())?);

    let x = Uint64::new(5.into())?;
    let y = Uint64::new(0.into())?;
    let z = (x + y)?;

    assert_eq!(z, Uint64::new(5.into())?);

    let x = Uint64::new(18446744073709551614_u64.into())?;
    let y = Uint64::new(1.into())?;
    let z = (x + y)?;

    assert_eq!(z, Uint64::new(18446744073709551615_u64.into())?);

    Ok(())
}

#[test]
fn subs_uint64() -> Result<(), Box<dyn std::error::Error>> {
    let x = Uint64::new(42.into())?;
    let y = Uint64::new(7.into())?;
    let z = (x - y)?;

    assert_eq!(z, Uint64::new(35.into())?);

    let x = Uint64::new(5.into())?;
    let y = Uint64::new(0.into())?;
    let z = (x - y)?;

    assert_eq!(z, Uint64::new(5.into())?);

    let x = Uint64::new(18446744073709551615_u64.into())?;
    let y = Uint64::new(1.into())?;
    let z = (x - y)?;

    assert_eq!(z, Uint64::new(18446744073709551614_u64.into())?);

    Ok(())
}

#[test]
fn muls_uint64() -> Result<(), Box<dyn std::error::Error>> {
    let x = Uint64::new(42.into())?;
    let y = Uint64::new(7.into())?;
    let z = (x * y)?;

    assert_eq!(z, Uint64::new(294.into())?);

    let x = Uint64::new(5.into())?;
    let y = Uint64::new(0.into())?;
    let z = (x * y)?;

    assert_eq!(z, Uint64::new(0.into())?);

    let x = Uint64::new(18446744073709551615_u64.into())?;
    let y = Uint64::new(1.into())?;
    let z = (x * y)?;

    assert_eq!(z, Uint64::new(18446744073709551615_u64.into())?);

    Ok(())
}

#[test]
fn divs_uint64() -> Result<(), Box<dyn std::error::Error>> {
    let x = Uint64::new(42.into())?;
    let y = Uint64::new(7.into())?;
    let z = (x / y)?;

    assert_eq!(z, Uint64::new(6.into())?);

    let x = Uint64::new(0.into())?;
    let y = Uint64::new(5.into())?;
    let z = (x / y)?;

    assert_eq!(z, Uint64::new(0.into())?);

    let x = Uint64::new(44.into())?;
    let y = Uint64::new(7.into())?;
    let z = (x / y)?;

    assert_eq!(z, Uint64::new(6.into())?);

    let x = Uint64::new(18446744073709551615_u64.into())?;
    let y = Uint64::new(1.into())?;
    let z = (x / y)?;

    assert_eq!(z, Uint64::new(18446744073709551615_u64.into())?);

    Ok(())
}

#[test]
fn rems_uint64() -> Result<(), Box<dyn std::error::Error>> {
    let x = Uint64::new(42.into())?;
    let y = Uint64::new(7.into())?;
    let z = (x % y)?;

    assert_eq!(z, Uint64::new(0.into())?);

    let x = Uint64::new(0.into())?;
    let y = Uint64::new(5.into())?;
    let z = (x % y)?;

    assert_eq!(z, Uint64::new(0.into())?);

    let x = Uint64::new(44.into())?;
    let y = Uint64::new(7.into())?;
    let z = (x % y)?;

    assert_eq!(z, Uint64::new(2.into())?);

    let x = Uint64::new(18446744073709551615_u64.into())?;
    let y = Uint64::new(2.into())?;
    let z = (x % y)?;

    assert_eq!(z, Uint64::new(1.into())?);

    Ok(())
}
