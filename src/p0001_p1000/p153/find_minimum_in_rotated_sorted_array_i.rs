// <Array, Binary Search (Exclusive Bounds)>
// Time: O(log n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 || nums[0] < nums[nums.len() - 1] {
            return nums[0];
        }
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = (left + right) >> 1;
            if nums[mid] < nums[0] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        nums[left]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![3, 4, 5, 1, 2];
        let result = Solution::find_min(nums);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let result = Solution::find_min(nums);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![11, 13, 15, 17];
        let result = Solution::find_min(nums);
        let expected = 11;
        assert_eq!(result, expected);
    }
}
