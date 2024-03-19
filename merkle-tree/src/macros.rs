/// The macro that checkes whether two given indexes are sibling indexes in a Merkle tree or not.
///
/// # Examples
///
/// Check whether 0 and 1 are sibling indexes:
/// ```rs
/// let is_sibling = is_sibling!(0, 1); // true
/// ```
///
/// Check whether 1 and 2 are sibling indexes:
/// ```rs
/// let is_sibling = is_sibling!(1, 2); // false
/// ```
///
/// Check whether 44 and 45 are sibling indexes:
/// ```rs
///  let is_sibling = is_sibling!(44, 45); // true
/// ```
///
/// Check whether 657 and 658 are sibling indexes:
/// ```rs
///  let is_sibling = is_sibling!(657, 658); // true
/// ```
///
macro_rules! is_sibling {
    ($x:expr, $y:expr) => {{
        ($x % 2 == 0 && $y == $x + 1) || ($y % 2 == 0 && $x == $y + 1)
    }};
}

pub(crate) use is_sibling;

/// The macro that whether swaps two `Field` elements or not based on the given condition.
///
/// # Examples
///
/// Swap seven and two:
/// ```rs
/// let (seven, two) = swap!(true, Field::from(2), Field::from(7));
/// ```
///
/// Don't swap three and six:
/// ```rs
/// let (three, six) = swap!(false, Field::from(3), Field::from(6));
/// ```
///
macro_rules! swap {
    ($c:expr, $x:expr, $y:expr) => {{
        let m = Field::from($c as u8) * ($x - $y);
        let a = $x - m;
        let b = $y + m;
        (a, b)
    }};
}

pub(crate) use swap;

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
macro_rules! choose {
    ($c:expr, $x:expr, $y:expr) => {{
        let m = Field::from($c as u8);

        let one: Field = 1.into();

        $x * m + $y * (one - m)
    }};
}

pub(crate) use choose;
