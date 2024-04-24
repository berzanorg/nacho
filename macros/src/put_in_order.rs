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
#[macro_export]
macro_rules! put_in_order {
    ($c:expr, &[$x:expr, $y:expr]) => {{
        let m = Field::from($c as u8) * ($x - $y);
        let a = $x - m;
        let b = $y + m;
        &[a, b]
    }};
}
