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

/// The macro that whether puts two `Field` elements in order based on the given condition.
///
/// # Examples
///
/// Put seven and two in order:
/// ```rs
/// let &[seven, two] = put_in_order!(true, Field::from(2), Field::from(7));
/// ```
///
/// Don't put three and six in order:
/// ```rs
/// let &[three, six] = put_in_order!(false, Field::from(3), Field::from(6));
/// ```
///
macro_rules! put_in_order {
    ($c:expr, &[$x:expr, $y:expr]) => {{
        let m = Field::from($c as u8) * ($x - $y);
        let a = $x - m;
        let b = $y + m;
        &[a, b]
    }};
}

pub(crate) use put_in_order;

/// The macro that picks between two `u64` elements based on the given condition.
///
/// # Examples
///
/// Pick first element:
/// ```rs
/// let two = pick!(true, 2, 7);
/// ```
///
/// Pick second element:
/// ```rs
/// let six = pick!(false, 3, 6);
/// ```
///
macro_rules! pick {
    ($c:expr, $x:expr, $y:expr) => {{
        let m = $c as u64;

        $x * m + $y * (1 - m)
    }};
}

pub(crate) use pick;
