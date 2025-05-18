// <Idiomatic>
// Time: O(n)
// Space: O(n)

use crate::utils::to_clean_chars::ToCleanChars;

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

// <Two Pointers>
// Time: O(n)
// Space: O(n)

pub struct Solution2;

impl Solution2 {
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
        let result = Solution::is_palindrome(s.clone());
        let result2 = Solution2::is_palindrome(s);

        assert_eq!(result, true);
        assert_eq!(result2, true);
    }

    #[test]
    fn test_case_2() {
        let s = String::from("race a car");
        let result = Solution::is_palindrome(s.clone());
        let result2 = Solution2::is_palindrome(s);

        assert_eq!(result, false);
        assert_eq!(result2, false);
    }

    #[test]
    fn test_case_3() {
        let s = String::from(" ");
        let result = Solution::is_palindrome(s.clone());
        let result2 = Solution2::is_palindrome(s);

        assert_eq!(result, true);
        assert_eq!(result2, true);
    }
}
