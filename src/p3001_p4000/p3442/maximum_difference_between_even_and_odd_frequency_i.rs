use std::cmp::{max, min};

// <Hash Table, String>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut freq = vec![0; 26];

        for &byte in s.as_bytes() {
            freq[(byte - b'a') as usize] += 1;
        }

        let mut odd = 0;
        let mut even = i32::MAX;

        for &n in &freq {
            if n % 2 == 1 {
                odd = max(odd, n);
            } else if n != 0 {
                even = min(even, n);
            }
        }

        if even == i32::MAX { odd } else { odd - even }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "aaaaabbc".to_string();
        let result = Solution::max_difference(s);
        let expected = 3;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let s = "abcabcab".to_string();
        let result = Solution::max_difference(s);
        let expected = 1;

        assert_eq!(result, expected);
    }
}
