use std::cmp::Ordering;

// <Array, Bit Manipulation>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut cur = 1;
        let mut ans = 1;
        for &num in nums.iter().skip(1) {
            match num.cmp(&max) {
                Ordering::Less => {
                    cur = 0;
                },
                Ordering::Equal => {
                    cur += 1;
                    ans = ans.max(cur);
                },
                Ordering::Greater => {
                    max = num;
                    cur = 1;
                    ans = 1;
                },
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 2, 3, 3, 2, 2];
        let result = Solution::longest_subarray(nums);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1, 2, 3, 4];
        let result = Solution::longest_subarray(nums);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_46() {
        let nums = vec![
            96317, 96317, 96317, 96317, 96317, 96317, 96317, 96317, 96317, 279979,
        ];
        let result = Solution::longest_subarray(nums);
        let expected = 1;
        assert_eq!(result, expected);
    }
}
