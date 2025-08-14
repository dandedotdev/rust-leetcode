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
        let num = String::from("6777133339");
        let expected = String::from("777");
        assert_eq!(Solution::largest_good_integer(num), expected);
    }

    #[test]
    fn test_case_2() {
        let num = String::from("2300019");
        let expected = String::from("000");
        assert_eq!(Solution::largest_good_integer(num), expected);
    }

    #[test]
    fn test_case_3() {
        let num = String::from("42352338");
        let expected = String::from("");
        assert_eq!(Solution::largest_good_integer(num), expected);
    }
}
