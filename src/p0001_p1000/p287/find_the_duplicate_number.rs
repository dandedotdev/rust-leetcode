// <Array, Binary Search, Two Pointers>
// Time: O(n)
// Space: O(1)

// Floyd Cycle Detection Algorithm

pub struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let (mut slow, mut fast) = (0, 0);
        loop {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
            if slow == fast {
                break;
            }
        }
        let mut cur = 0;
        loop {
            slow = nums[slow as usize];
            cur = nums[cur as usize];
            if slow == cur {
                break;
            }
        }
        cur
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 3, 4, 2, 2];
        let result = Solution::find_duplicate(nums);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![3, 1, 3, 4, 2];
        let result = Solution::find_duplicate(nums);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![3, 3, 3, 3, 3];
        let result = Solution::find_duplicate(nums);
        let expected = 3;
        assert_eq!(result, expected);
    }
}
