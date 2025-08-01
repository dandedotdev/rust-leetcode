// <Array, Prefix Sum>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];

        {
            let mut prefix = vec![1; n];
            let mut suffix = vec![1; n];

            for i in 1..n {
                prefix[i] = prefix[i - 1] * nums[i - 1];
            }

            for i in (0..n - 1).rev() {
                suffix[i] = suffix[i + 1] * nums[i + 1];
            }

            for i in 0..n {
                result[i] = prefix[i] * suffix[i];
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 2, 3, 4];
        let result = Solution::product_except_self(nums);
        let expected = vec![24, 12, 8, 6];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let result = Solution::product_except_self(nums);
        let expected = vec![0, 0, 9, 0, 0];

        assert_eq!(result, expected);
    }
}
