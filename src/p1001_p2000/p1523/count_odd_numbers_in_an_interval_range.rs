// <Math>
// Time: O(1)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        ((high - low) >> 1) + ((high & 1) | (low & 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let low = 3;
        let high = 7;
        let result = Solution::count_odds(low, high);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_case_2() {
        let low = 8;
        let high = 10;
        let result = Solution::count_odds(low, high);
        assert_eq!(result, 1);
    }
}
