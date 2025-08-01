use std::cmp::Ordering;

// <Array, Two Pointers>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        if numbers.is_empty() {
            return vec![];
        }

        let (mut left, mut right) = (0, numbers.len() - 1);

        loop {
            match (numbers[left] + numbers[right]).cmp(&target) {
                Ordering::Equal => {
                    return vec![(left + 1) as i32, (right + 1) as i32];
                }
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(numbers, target);
        let expected = vec![1, 2];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![2, 3, 4];
        let target = 6;
        let result = Solution::two_sum(numbers, target);
        let expected = vec![1, 3];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let numbers = vec![-1, 0];
        let target = -1;
        let result = Solution::two_sum(numbers, target);
        let expected = vec![1, 2];

        assert_eq!(result, expected);
    }
}
