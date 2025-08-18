use std::cmp::Ordering;

// <Array, Binary Search>
// Time: O(log n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);

        while left <= right {
            let mid = (left + right) >> 1;

            if nums[mid] == target {
                return mid as i32;
            }

            match nums[left].cmp(&nums[mid]) {
                Ordering::Less | Ordering::Equal => {
                    if nums[left] <= target && target < nums[mid] {
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                    }
                }
                Ordering::Greater => {
                    if nums[mid] < target && target <= nums[right] {
                        left = mid + 1;
                    } else {
                        right = mid - 1;
                    }
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        let result = Solution::search(nums, target);
        let expected = 4;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        let result = Solution::search(nums, target);
        let expected = -1;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![1];
        let target = 0;
        let result = Solution::search(nums, target);
        let expected = -1;

        assert_eq!(result, expected);
    }
}
