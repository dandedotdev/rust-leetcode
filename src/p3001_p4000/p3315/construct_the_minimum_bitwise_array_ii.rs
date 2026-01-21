// <Array, Bit Manipulation>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .map(|n| {
                if n & 1 == 0 {
                    -1
                } else {
                    n - (1 << (n.trailing_ones() - 1))
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![2, 3, 5, 7];
        let result = Solution::min_bitwise_array(nums);
        let expected = vec![-1, 1, 4, 3];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![11, 13, 31];
        let result = Solution::min_bitwise_array(nums);
        let expected = vec![9, 12, 15];
        assert_eq!(result, expected);
    }
}
