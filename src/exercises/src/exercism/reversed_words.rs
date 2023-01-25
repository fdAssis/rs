/**
 * Words are separated by exactly one space and there are no
 * leading or trailing spaces.
 *
 * Example:
 * "The greatest victory is that which requires no battle" --> "battle no requires which that is victory greatest The"
 */

fn reverse_words(words: &str) -> String {
    "backward! is This".to_string()
}

fn main() {
    let quote = "The greatest victory is that which requires no battle";

    println!("{}", reverse_words(&quote));
}

#[cfg(test)]
mod test {
    #[test]
    fn reverse_words_test() {
        let quote = "The greatest victory is that which requires no battle";

        assert_eq!(quote, reverse_words(quote));
    }
}
