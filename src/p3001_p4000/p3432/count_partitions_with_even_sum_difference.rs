// <Array, Math, Prefix Sum>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let total_sum: i32 = nums.iter().sum();
        if total_sum % 2 == 0 {
            nums.len() as i32 - 1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![10, 10, 3, 7, 6];
        let result = Solution::count_partitions(nums);
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1, 2, 2];
        let result = Solution::count_partitions(nums);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![2, 4, 6, 8];
        let result = Solution::count_partitions(nums);
        let expected = 3;
        assert_eq!(result, expected);
    }
}
