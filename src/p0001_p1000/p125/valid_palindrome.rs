use crate::utils::to_clean_chars::ToCleanChars;

// <Idiomatic>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars = s.to_clean_chars();
        let len = chars.len();
        let first_half = &chars[..len / 2];
        let second_half = &chars[len - len / 2..];
        for (x, y) in first_half.iter().rev().zip(second_half) {
            if x != y {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "A man, a plan, a canal: Panama".to_string();
        let result = Solution::is_palindrome(s);
        assert!(result);
    }

    #[test]
    fn test_case_2() {
        let s = "race a car".to_string();
        let result = Solution::is_palindrome(s);
        assert!(!result);
    }

    #[test]
    fn test_case_3() {
        let s = " ".to_string();
        let result = Solution::is_palindrome(s);
        assert!(result);
    }
}
