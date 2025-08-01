// <Array, Sorting>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut current = 1;
        let mut max = 0;

        nums.sort_unstable();

        for i in 1..nums.len() {
            if nums[i - 1] == nums[i] {
                continue;
            }

            if nums[i - 1] + 1 == nums[i] {
                current += 1;
            } else {
                max = max.max(current);
                current = 1;
            }
        }

        max.max(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        let result = Solution::longest_consecutive(nums);
        let expected = 4;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        let result = Solution::longest_consecutive(nums);
        let expected = 9;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![1, 0, 1, 2];
        let result = Solution::longest_consecutive(nums);
        let expected = 3;

        assert_eq!(result, expected);
    }
}
