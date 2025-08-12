use std::collections::VecDeque;

// <Array, Queue, Sliding Window, Heap(Priority Queue), Monotonic Queue>
// Time: O(n)
// Space: O(k)

pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut deque = VecDeque::new();
        let mut result = vec![];

        for i in 0..nums.len() {
            if let Some(&idx_front) = deque.front()
                && idx_front + k <= i
            {
                deque.pop_front();
            }

            while let Some(&idx_back) = deque.back() {
                if nums[idx_back] < nums[i] {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            deque.push_back(i);

            if i + 1 >= k {
                result.push(nums[*deque.front().unwrap()]);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let result = Solution::max_sliding_window(nums, k);
        let expected = vec![3, 3, 5, 5, 6, 7];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1];
        let k = 1;
        let result = Solution::max_sliding_window(nums, k);
        let expected = vec![1];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_46() {
        let nums = vec![4, -2];
        let k = 2;
        let result = Solution::max_sliding_window(nums, k);
        let expected = vec![4];

        assert_eq!(result, expected);
    }
}
