/// The macro that chooses between two `Field` elements based on the given condition.
///
/// # Examples
///
/// Choose first element:
/// ```rs
/// let two = choose!(true, Field::from(2), Field::from(7));
/// ```
///
/// Choose second element:
/// ```rs
/// let six = choose!(false, Field::from(3), Field::from(6));
/// ```
///
#[macro_export]
macro_rules! choose {
    ($c:expr, $x:expr, $y:expr) => {{
        let m = Field::from($c as u8);

        let one: Field = 1.into();

        $x * m + $y * (one - m)
    }};
}
