use crate::p0001_p1000::p191::number_of_1_bits::Solution as NumberOf1BitsSolution;

// <Bit Manipulation, Dynamic Programming>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        (0..=n).map(NumberOf1BitsSolution::hamming_weight).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 2;
        let result = Solution::count_bits(n);
        let expected = vec![0, 1, 1];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let n = 5;
        let result = Solution::count_bits(n);
        let expected = vec![0, 1, 1, 2, 1, 2];
        assert_eq!(result, expected);
    }
}
