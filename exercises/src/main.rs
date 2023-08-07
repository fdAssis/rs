mod codewars;
use crate::codewars::{beginner_series::get_sum, reversed_words::reverse_words};

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reverse_words_test() {
        let quote = "The greatest victory is that which requires no battle";
        let expect = "battle no requires which that is victory greatest The";

        assert_eq!(expect, reverse_words(quote));
    }

    #[test]
    fn beginner_series_test() {
        assert_eq!(get_sum(0, 1), 1);
        assert_eq!(get_sum(1, 2), 3);
        assert_eq!(get_sum(5, -1), 14);
        assert_eq!(get_sum(505, 4), 127759);
    }
}
