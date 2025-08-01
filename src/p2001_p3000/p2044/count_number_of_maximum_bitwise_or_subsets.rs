// <Array, Backtracking, Bit Manipulation>
// Time: O(2^n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let possible_max = nums.iter().fold(0, |acc, &v| acc | v);

        Self::backtrack(&nums, 0, possible_max, 0)
    }

    pub fn backtrack(
        nums: &[i32],
        idx: usize,
        max_bitwise_or: i32,
        current_bitwise_or: i32,
    ) -> i32 {
        if current_bitwise_or == max_bitwise_or {
            // the number of subsets that can be formed with the remaining elements
            return 1 << (nums.len() - idx);
        }

        if idx == nums.len() {
            return 0;
        }

        Self::backtrack(
            nums,
            idx + 1,
            max_bitwise_or,
            current_bitwise_or | nums[idx],
        ) + Self::backtrack(nums, idx + 1, max_bitwise_or, current_bitwise_or)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![3, 1];
        let result = Solution::count_max_or_subsets(nums);
        let expected = 2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![2, 2, 2];
        let result = Solution::count_max_or_subsets(nums);
        let expected = 7;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![3, 2, 1, 5];
        let result = Solution::count_max_or_subsets(nums);
        let expected = 6;

        assert_eq!(result, expected);
    }
}
