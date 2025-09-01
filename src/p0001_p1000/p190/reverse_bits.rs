// <Bit Manipulation, Divide and Conquer>
// Time: O(1)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        (0..32).fold(0, |ans, bit| (ans << 1) | ((n >> bit) & 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 43261596;
        let result = Solution::reverse_bits(n);
        let expected = 964176192;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let n = 2147483644;
        let result = Solution::reverse_bits(n);
        let expected = 1073741822;
        assert_eq!(result, expected);
    }
}
