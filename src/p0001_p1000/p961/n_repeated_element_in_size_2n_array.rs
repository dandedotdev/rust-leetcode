use std::collections::HashSet;

// <Array, Hash Table>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for x in nums {
            if !set.insert(x) {
                return x;
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
        let nums = vec![1, 2, 3, 3];
        let result = Solution::repeated_n_times(nums);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![2, 1, 2, 5, 3, 2];
        let result = Solution::repeated_n_times(nums);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![5, 1, 5, 2, 5, 3, 5, 4];
        let result = Solution::repeated_n_times(nums);
        let expected = 5;
        assert_eq!(result, expected);
    }
}
