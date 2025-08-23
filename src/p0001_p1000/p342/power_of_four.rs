// <Bit Manipulation, Math, Recursion>
// Time: O(log n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn is_power_of_four(mut num: i32) -> bool {
        if num <= 0 {
            return false;
        }
        while num % 4 == 0 {
            num /= 4;
        }
        num == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let num = 16;
        let result = Solution::is_power_of_four(num);
        assert!(result);
    }

    #[test]
    fn test_case_2() {
        let num = 5;
        let result = Solution::is_power_of_four(num);
        assert!(!result);
    }

    #[test]
    fn test_case_3() {
        let num = 1;
        let result = Solution::is_power_of_four(num);
        assert!(result);
    }
}
