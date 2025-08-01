// <Array>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        nums.iter()
            .zip(nums.iter().cycle().skip(1))
            .fold(0, |max, (prev, next)| max.max((prev - next).abs()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 2, 4];
        let result = Solution::max_adjacent_distance(nums);
        let expected = 3;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![-5, -10, -5];
        let result = Solution::max_adjacent_distance(nums);
        let expected = 5;

        assert_eq!(result, expected);
    }
}
