// <Array, Bit Manipulation, Math, Sliding Window>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let mut bit_index: [i32; 30] = [0; 30]; // The first index of the bit in the nums
        let mut result: Vec<i32> = vec![1; nums.len()];

        for (index, &num) in nums.iter().enumerate().rev().skip_while(|(_, v)| **v == 0) {
            for (b, v) in bit_index.iter_mut().enumerate() {
                let bit = (num >> b) & 1;

                *v = (1 - bit) * *v + bit * (index as i32);
            }

            let max_position = *bit_index.iter().max().unwrap();

            result[index] = max_position - index as i32 + 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 0, 2, 1, 3];
        let result = Solution::smallest_subarrays(nums);

        assert_eq!(result, vec![3, 3, 2, 2, 1]);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1, 2];
        let result = Solution::smallest_subarrays(nums);

        assert_eq!(result, vec![2, 1]);
    }
}
