// <Array, Binary Search>
// Time: O(log n)
// Space: O(1)

// Idiomatic

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(n) => n as i32,
            Err(_e) => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        let result = Solution::search(nums, target);
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let result = Solution::search(nums, target);
        let expected = -1;
        assert_eq!(result, expected);
    }
}
