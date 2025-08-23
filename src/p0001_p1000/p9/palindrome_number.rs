// <Math>
// Time: O(log10(n))
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut rem = x;
        let mut y = 0; // y for `rev`
        while rem > 0 {
            y = y * 10 + rem % 10;
            rem /= 10;
        }
        x == y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let x = 121;
        let result = Solution::is_palindrome(x);
        assert!(result);
    }

    #[test]
    fn test_case_2() {
        let x = -121;
        let result = Solution::is_palindrome(x);
        assert!(!result);
    }

    #[test]
    fn test_case_3() {
        let x = 10;
        let result = Solution::is_palindrome(x);
        assert!(!result);
    }
}
