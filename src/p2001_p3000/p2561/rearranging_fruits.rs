use std::collections::HashMap;

// <Array, Hash Table, Greedy, Sort>
// Time: O(n log n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        let mut min_cost = i32::MAX;
        for &fruit in basket1.iter() {
            *memo.entry(fruit).or_default() += 1;
            min_cost = min_cost.min(fruit);
        }
        for &fruit in basket2.iter() {
            *memo.entry(fruit).or_default() -= 1;
            min_cost = min_cost.min(fruit);
        }
        let mut to_swap = Vec::new();
        for (&fruit, &diff) in memo.iter() {
            if diff % 2 != 0 {
                return -1;
            }
            for _ in 0..(diff.abs() / 2) {
                to_swap.push(fruit);
            }
        }
        to_swap.sort_unstable();
        to_swap
            .iter()
            .take(to_swap.len() / 2)
            // directly swap vs. swap with min_cost fruit
            .map(|&fruit| fruit.min(2 * min_cost) as i64)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let basket1 = vec![4, 2, 2, 2];
        let basket2 = vec![1, 4, 1, 2];
        let result = Solution::min_cost(basket1, basket2);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let basket1 = vec![2, 3, 4, 1];
        let basket2 = vec![3, 2, 5, 1];
        let result = Solution::min_cost(basket1, basket2);
        let expected = -1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_24() {
        let basket1 = vec![4, 4, 4, 4, 3];
        let basket2 = vec![5, 5, 5, 5, 3];
        let result = Solution::min_cost(basket1, basket2);
        let expected = 8;
        assert_eq!(result, expected);
    }
}
