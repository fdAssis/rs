/**
 * Given two integers a and b, which can be positive or negative, find the sum
 * of all the integers between and including them and return it.
 * If the two numbers are equal return a or b.
 *
 * Note: a and b are not ordered!
 */
use std::cmp::Ordering;

pub fn get_sum(a: i64, b: i64) -> i64 {
    match a.cmp(&b) {
        Ordering::Less => return (a..=b).sum(),
        Ordering::Greater => return (b..=a).sum(),
        Ordering::Equal => return a,
    }
}
