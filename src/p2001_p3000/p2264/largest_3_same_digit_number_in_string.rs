// <String>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut ans: String = String::default();
        let bytes = num.as_bytes();
        for i in 0..bytes.len() - 2 {
            if bytes[i] == bytes[i + 1] && bytes[i] == bytes[i + 2] {
                let cur = &num[i..i + 3];
                if cur > &ans {
                    ans = cur.to_string();
                }
            }
        }
        ans
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
