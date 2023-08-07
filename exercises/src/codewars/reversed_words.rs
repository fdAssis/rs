/**
 * Words are separated by exactly one space and there are no
 * leading or trailing spaces.
 *
 * Example:
 * "The greatest victory is that which requires no battle" --> "battle no requires which that is victory greatest The"
 */

pub fn reverse_words(words: &str) -> String {
    let mut result: String = String::from("");

    for i in words.split_whitespace().rev() {
        result.push_str(format!("{i}{}", " ").as_str());
    }

    result.trim().to_string()
}
