//! Constraint: Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

// <Math>
// Time: O(log(k))
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut result: i32 = 0;
        let mut num = x;

        while num != 0 {
            let digit = num % 10;
            num /= 10;

            // faster than `checked_mul(10)`
            if result > i32::MAX / 10 || (result == i32::MAX / 10 && digit > 7) {
                return 0;
            }

            // faster than `checked_add(digit)`
            if result < i32::MIN / 10 || (result == i32::MIN / 10 && digit < -8) {
                return 0;
            }

            result = result * 10 + digit;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let x = 123;
        let result = Solution::reverse(x);
        let expected = 321;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let x = -123;
        let result = Solution::reverse(x);
        let expected = -321;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let x = 120;
        let result = Solution::reverse(x);
        let expected = 21;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_1036() {
        let x = 1534236469;
        let result = Solution::reverse(x);
        let expected = 0;

        assert_eq!(result, expected);
    }
}
