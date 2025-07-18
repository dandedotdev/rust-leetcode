// <Array, Sorting>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut current = 1;
        let mut max = 0;

        nums.sort_unstable();

        for i in 1..nums.len() {
            if nums[i - 1] == nums[i] {
                continue;
            }

            if nums[i - 1] + 1 == nums[i] {
                current += 1;
            } else {
                max = max.max(current);
                current = 1;
            }
        }

        max.max(current)
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
