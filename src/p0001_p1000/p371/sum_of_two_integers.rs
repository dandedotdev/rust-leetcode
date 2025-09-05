// <Bit Manipulation>
// Time: O(1)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let sum_without_carry = a ^ b; // XOR: 0 -> 1, 1 -> 0
            let carry = (a & b) << 1;
            a = sum_without_carry;
            b = carry;
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let result = Solution::get_sum(1, 2);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let result = Solution::get_sum(2, 3);
        let expected = 5;
        assert_eq!(result, expected);
    }
}
