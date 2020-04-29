//! # List14-1
//! 
//! `list14-1` crate is a collections of utilities to make performing certain calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = list14-1::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
