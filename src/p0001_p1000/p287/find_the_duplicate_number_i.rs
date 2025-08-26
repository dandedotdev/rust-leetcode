use std::collections::HashSet;

// <Array, Binary Search, Two Pointers>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut memo = HashSet::new();
        for num in nums {
            if memo.contains(&num) {
                return num;
            }
            memo.insert(num);
        }
        -1
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
