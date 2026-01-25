// <Array, Sliding Window, Sorting>
// Time: O(n log n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 {
            return 0;
        }
        nums.sort_unstable();
        nums.iter()
            .zip(nums[(k - 1) as usize..].iter())
            .map(|(a, b)| b - a)
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![90];
        let k = 1;
        let result = Solution::minimum_difference(nums, k);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![9, 4, 1, 7];
        let k = 2;
        let result = Solution::minimum_difference(nums, k);
        let expected = 2;
        assert_eq!(result, expected);
    }
}
