/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
///
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn main() {
    println!("1 + 1 = {}", add_one(1));
}
