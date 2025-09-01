// <Array, Binary Search, Bit Manipulation, Hash, Math, Sorting>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .enumerate()
            .fold(0, |acc, (idx, num)| acc ^ (idx + 1) as i32 ^ num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![3, 0, 1];
        let result = Solution::missing_number(nums);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![0, 1];
        let result = Solution::missing_number(nums);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        let result = Solution::missing_number(nums);
        let expected = 8;
        assert_eq!(result, expected);
    }
}
