use std::collections::HashSet;

// <Array, Hash Table, Greedy>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let set: HashSet<_> = nums.iter().cloned().collect();
        let sum: i32 = set.iter().filter(|&&v| v > 0).sum();

        if sum != 0 {
            return sum;
        }

        *nums.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = Solution::max_sum(nums);

        assert_eq!(result, 15);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1, 1, 0, 1, 1];
        let result = Solution::max_sum(nums);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![1, 2, -1, -2, 1, 0, -1];
        let result = Solution::max_sum(nums);

        assert_eq!(result, 3);
    }
}
