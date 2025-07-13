use std::collections::HashMap;

// <Array, Hash Table, Sorting>
// Time: O(n log n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut pairs: Vec<_> = map.into_iter().collect();

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
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
