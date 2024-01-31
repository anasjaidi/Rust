use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut hash_s: HashMap<char, i32> = HashMap::new();

        // Increment the frequency of characters in s
        for ch in s.chars() {
            *hash_s.entry(ch).or_insert(0) += 1;
        }

        // Decrement the frequency of characters in t
        for ch in t.chars() {
            if let Some(count) = hash_s.get_mut(&ch) {
                *count -= 1;
                if *count == 0 {
                    hash_s.remove(&ch);
                }
            } else {
                return false; // Found a character in t that is not in s
            }
        }

        // If the HashMap is empty, all characters in s and t have canceled each other out
        hash_s.is_empty()
    }
}

