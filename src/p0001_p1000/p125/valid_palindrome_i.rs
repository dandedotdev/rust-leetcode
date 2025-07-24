use crate::utils::to_clean_chars::ToCleanChars;

// <Two Pointers>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars = s.to_clean_chars();

        let mut left = 0;
        let mut right = chars.len().saturating_sub(1);

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
        let s = String::from("A man, a plan, a canal: Panama");
        let result = Solution::is_palindrome(s);

        assert!(result);
    }

    #[test]
    fn test_case_2() {
        let s = String::from("race a car");
        let result = Solution::is_palindrome(s);

        assert!(!result);
    }

    #[test]
    fn test_case_3() {
        let s = String::from(" ");
        let result = Solution::is_palindrome(s);

        assert!(result);
    }
}
