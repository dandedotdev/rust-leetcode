// <Bit Manipulation, Math, Recursion>
// Time: O(1)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        num > 0 && (num & (num - 1)) == 0 && (num & 0x55555555) != 0
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
