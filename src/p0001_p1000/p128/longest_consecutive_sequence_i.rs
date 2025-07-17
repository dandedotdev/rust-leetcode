use std::collections::HashSet;

// <Array, Hash Table>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let num_set: HashSet<_> = nums.into_iter().collect();
        let mut result = 0;

        for &num in &num_set {
            if !num_set.contains(&(num - 1)) {
                let count = (num..).take_while(|x| num_set.contains(x)).count();

                result = result.max(count);
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::longest_consecutive(vec![1, 0, 1, 2]), 3);
    }
}
