// <Bit Manipulation, Map>
// Time: O(right - left)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        (left..=right).fold(0, |acc, x| {
            acc + match x.count_ones() {
                2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 | 23 | 29 | 31 => 1,
                _ => 0,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let result = Solution::count_prime_set_bits(6, 10);
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let result = Solution::count_prime_set_bits(10, 15);
        let expected = 5;
        assert_eq!(result, expected);
    }
}
