use crate::constants::vowels::VOWELS;

// <String>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3 {
            return false;
        }
        let mut has_vowel = false;
        let mut has_consonant = false;
        for char in word.chars() {
            if !char.is_ascii_alphanumeric() {
                return false;
            }
            let char = char.to_ascii_lowercase();
            if VOWELS.contains(&char) {
                has_vowel = true;
            } else if char.is_ascii_alphabetic() {
                has_consonant = true;
            }
        }
        has_vowel && has_consonant
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let word = "234Adas".to_string();
        let result = Solution::is_valid(word);
        assert!(result);
    }

    #[test]
    fn test_case_2() {
        let word = "b3".to_string();
        let result = Solution::is_valid(word);
        assert!(!result);
    }

    #[test]
    fn test_case_3() {
        let word = "a3$e".to_string();
        let result = Solution::is_valid(word);
        assert!(!result);
    }
}
