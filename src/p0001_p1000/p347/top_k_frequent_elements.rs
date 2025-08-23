use std::collections::HashMap;

// <Array, Hash Table, Sorting>
// Time: O(n log n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            *memo.entry(num).or_default() += 1;
        }
        let mut pairs: Vec<_> = memo.into_iter().collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1));
        pairs
            .into_iter()
            .take(k as usize)
            .map(|(num, _)| num)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let result = Solution::top_k_frequent(nums, k);
        let expected = vec![1, 2];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1];
        let k = 1;
        let result = Solution::top_k_frequent(nums, k);
        let expected = vec![1];
        assert_eq!(result, expected);
    }
}
