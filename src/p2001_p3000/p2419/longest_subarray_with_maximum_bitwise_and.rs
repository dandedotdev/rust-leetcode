use std::cmp::Ordering;

// <Array, Bit Manipulation>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut current = 1;
        let mut result = 1;

        for &num in nums.iter().skip(1) {
            match num.cmp(&max) {
                Ordering::Less => {
                    current = 0;
                }
                Ordering::Equal => {
                    current += 1;
                    result = result.max(current);
                }
                Ordering::Greater => {
                    max = num;
                    current = 1;
                    result = 1;
                }
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
        let nums = vec![1, 2, 3, 3, 2, 2];
        let result = Solution::longest_subarray(nums);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1, 2, 3, 4];
        let result = Solution::longest_subarray(nums);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_case_46() {
        let nums = vec![
            96317, 96317, 96317, 96317, 96317, 96317, 96317, 96317, 96317, 279979,
        ];
        let result = Solution::longest_subarray(nums);

        assert_eq!(result, 1);
    }
}
