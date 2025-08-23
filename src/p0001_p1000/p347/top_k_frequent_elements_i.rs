use std::collections::{BinaryHeap, HashMap};

// <Array, Hash Table, Heap (Priority Queue)>
// Time: O(n log k)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            *memo.entry(num).or_default() += 1;
        }
        let mut heap = BinaryHeap::new();
        for (num, freq) in memo {
            heap.push((freq, num));
        }
        let mut ans = Vec::new();
        for _ in 0..k {
            if let Some((_, num)) = heap.pop() {
                ans.push(num);
            }
        }
        ans
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
