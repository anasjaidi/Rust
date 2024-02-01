impl Solution {
    pub fn is_palindrome(s: String) -> bool {
       let cleaned_string: String = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        let reversed_string: String = cleaned_string.chars().rev().collect();

        cleaned_string == reversed_string
    }
}
