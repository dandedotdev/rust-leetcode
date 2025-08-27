// <Bit Manipulation, Divide and Conquer>
// Time: O(1)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        n.count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 11;
        let result = Solution::hamming_weight(n);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let n = 128;
        let result = Solution::hamming_weight(n);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let n = 2147483645;
        let result = Solution::hamming_weight(n);
        let expected = 30;
        assert_eq!(result, expected);
    }
}
