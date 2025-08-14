// <String>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut result = String::default();
        let bytes = num.as_bytes();

        for i in 0..bytes.len() - 2 {
            if bytes[i] == bytes[i + 1] && bytes[i] == bytes[i + 2] {
                let cur = &num[i..i + 3];

                if cur > &result {
                    result = cur.to_string();
                }
            }
        }

        result
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
