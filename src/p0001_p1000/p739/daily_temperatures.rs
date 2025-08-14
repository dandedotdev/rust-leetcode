// <Array, Monotonic Stack, Stack>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut result = vec![0; n];
        let mut stack = Vec::with_capacity(n);

        for i in (0..n).rev() {
            while !stack.is_empty() && temperatures[i] >= temperatures[*stack.last().unwrap()] {
                stack.pop();
            }

            result[i] = if !stack.is_empty() {
                (*stack.last().unwrap() - i) as i32
            } else {
                0
            };

            stack.push(i);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let result = Solution::daily_temperatures(temperatures);
        let expected = vec![1, 1, 4, 2, 1, 1, 0, 0];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let temperatures = vec![30, 40, 50, 60];
        let result = Solution::daily_temperatures(temperatures);
        let expected = vec![1, 1, 1, 0];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let temperatures = vec![30, 60, 90];
        let result = Solution::daily_temperatures(temperatures);
        let expected = vec![1, 1, 0];

        assert_eq!(result, expected);
    }
}
