use std::collections::HashMap;

// <Array, Hash Table, Sorting>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut counts = HashMap::new();

        !nums.into_iter().all(|n| counts.insert(n, 1).is_none())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 2, 3, 1];
        let result = Solution::contains_duplicate(nums);

        assert_eq!(result, true);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1, 2, 3, 4];
        let result = Solution::contains_duplicate(nums);

        assert_eq!(result, false);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        let result = Solution::contains_duplicate(nums);

        assert_eq!(result, true);
    }
}
