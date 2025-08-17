use std::cmp::Ordering;

// <Array, Binary Search (Inclusive Bounds)>
// Time: O(log n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, (nums.len() - 1) as i32);

        while left <= right {
            let mid = (left + right) >> 1;

            match nums[mid as usize].cmp(&target) {
                Ordering::Equal => return mid,
                Ordering::Less => left = mid + 1,
                // `usize` is not allowed here for index out of bounds
                Ordering::Greater => right = mid - 1,
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
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        let result = Solution::search(nums, target);
        let expected = 4;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let result = Solution::search(nums, target);
        let expected = -1;

        assert_eq!(result, expected);
    }
}
