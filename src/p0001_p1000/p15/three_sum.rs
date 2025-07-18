use std::cmp::Ordering;

// <Array, Two Pointers, Sorting>
// Time: O(n^2)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut result = Vec::new();

        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut left, mut right) = (i + 1, n - 1);

            nums.sort_unstable();

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                match sum.cmp(&0) {
                    Ordering::Greater => right -= 1,
                    Ordering::Less => left += 1,
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[left], nums[right]]);
                        left += 1;

                        while left < right && nums[left] == nums[left - 1] {
                            left += 1;
                        }
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let test_case = vec![-1, 0, 1, 2, -1, -4];
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];

        assert_eq!(Solution::three_sum(test_case), expected);
    }

    #[test]
    fn test_case_2() {
        let test_case = vec![0, 1, 1];
        let expected: Vec<Vec<i32>> = vec![];

        assert_eq!(Solution::three_sum(test_case), expected);
    }

    #[test]
    fn test_case_3() {
        let test_case = vec![0, 0, 0];
        let expected = vec![vec![0, 0, 0]];

        assert_eq!(Solution::three_sum(test_case), expected);
    }
}
