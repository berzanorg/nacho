/// The macro that calculates the sibling index of the given index in a Merkle tree.
///
/// # Examples
///
/// Calculate sibling index of seven:
///
/// ```rs
/// let six = calculate_sibling_index!(7);
/// ```
///
/// Calculate sibling index of six:
///
/// ```rs
/// let seven = calculate_sibling_index!(6);
/// ```
///
#[macro_export]
macro_rules! calculate_sibling_index {
    ($x:expr) => {{
        if $x % 2 == 0 {
            $x + 1
        } else {
            $x - 1
        }
    }};
}
