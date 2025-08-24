// <Array, Dynamic Programming, Sliding Window>
// Time: O(n)
// Space: O(1)

// Bit Manipulation
pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let (mut left_idx, mut zero_count) = (0, 0);
        // window is the longest subarray with at most one 0
        (0..nums.len())
            .map(|i| {
                zero_count += nums[i] ^ 1; // XOR: 0 -> 1, 1 -> 0
                if zero_count > 1 {
                    zero_count -= nums[left_idx] ^ 1; // XOR: 0 -> 1, 1 -> 0
                    left_idx += 1;
                }
                i - left_idx
            })
            .max()
            .unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 1, 0, 1];
        let expected = 3;
        let actual = Solution::longest_subarray(nums);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
        let expected = 5;
        let actual = Solution::longest_subarray(nums);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![1, 1, 1];
        let expected = 2;
        let actual = Solution::longest_subarray(nums);
        assert_eq!(actual, expected);
    }
}
