// <Array, Bit Manipulation>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, i| acc ^ i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![2, 2, 1];
        let result = Solution::single_number(nums);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![4, 1, 2, 1, 2];
        let result = Solution::single_number(nums);
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![1];
        let result = Solution::single_number(nums);
        let expected = 1;
        assert_eq!(result, expected);
    }
}
