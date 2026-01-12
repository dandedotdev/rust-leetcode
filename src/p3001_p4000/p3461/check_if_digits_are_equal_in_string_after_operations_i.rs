// <Combinatorics, Math, Number Theory, Simulation, String>
// Time: O(n^2)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut digits: Vec<u8> = s.bytes().map(|b| b - b'0').collect();
        while digits.len() > 2 {
            digits = digits.windows(2)
                .map(|w| (w[0] + w[1]) % 10)
                .collect();
        }
        digits[0] == digits[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "3902".to_string();
        let result = Solution::has_same_digits(s);
        assert!(result);
    }

    #[test]
    fn test_case_2() {
        let s = "34789".to_string();
        let result = Solution::has_same_digits(s);
        assert!(!result);
    }
}
