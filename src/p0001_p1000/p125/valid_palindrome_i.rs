use crate::utils::to_clean_chars::ToCleanChars;

// <Two Pointers>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars = s.to_clean_chars();
        let (mut left, mut right) = (0, chars.len().saturating_sub(1));
        while left < right {
            if chars[left] != chars[right] {
                return false;
            }
            left += 1;
            right -= 1;
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
