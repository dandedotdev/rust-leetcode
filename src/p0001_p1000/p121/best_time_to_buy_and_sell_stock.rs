// <Array, Dynamic Programming>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut dp_i_0, mut dp_i_1) = (0, i32::MIN);
        for &price in &prices {
            dp_i_0 = dp_i_0.max(dp_i_1 + price); // Not holding stock
            dp_i_1 = dp_i_1.max(-price); // Holding stock
        }
        dp_i_0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let result = Solution::max_profit(prices);
        let expected = 5;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let prices = vec![7, 6, 4, 3, 1];
        let result = Solution::max_profit(prices);
        let expected = 0;
        assert_eq!(result, expected);
    }
}
