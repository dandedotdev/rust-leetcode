// <Array, Hash Table, Linked List, Heap (Priority Queue), Simulation, Doubly-Linked List, Ordered Set>
// Time: O(n^2)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        while !nums.windows(2).all(|w| w[0] <= w[1]) {
            if let Some((idx, sum)) = nums
                .windows(2)
                .enumerate()
                .map(|(i, w)| (i, w[0] + w[1]))
                .min_by_key(|&(_, sum)| sum)
            {
                nums[idx] = sum;
                nums.remove(idx + 1);
                ans += 1;
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
        let nums = vec![5, 2, 3, 1];
        let result = Solution::minimum_pair_removal(nums);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1, 2, 2];
        let result = Solution::minimum_pair_removal(nums);
        let expected = 0;
        assert_eq!(result, expected);
    }
}
