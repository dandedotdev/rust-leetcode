// <String>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let patterns = [
            "999", "888", "777", "666", "555", "444", "333", "222", "111", "000",
        ];
        for &p in patterns.iter() {
            if num.contains(p) {
                return p.to_string();
            }
        }
        String::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let num = "6777133339".to_string();
        let expected = "777".to_string();
        assert_eq!(Solution::largest_good_integer(num), expected);
    }

    #[test]
    fn test_case_2() {
        let num = "2300019".to_string();
        let expected = "000".to_string();
        assert_eq!(Solution::largest_good_integer(num), expected);
    }

    #[test]
    fn test_case_3() {
        let num = "42352338".to_string();
        let expected = "".to_string();
        assert_eq!(Solution::largest_good_integer(num), expected);
    }
}
