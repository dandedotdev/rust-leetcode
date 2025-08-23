// <Array, Two Pointers>
// Time: O(n)
// SPace: O(m)

const DELIMITER: char = ';';

pub struct Solution;

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut ans = String::default();
        for s in strs {
            ans.push_str(&format!("{}{}{}", s.len(), DELIMITER, s));
        }
        ans
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut ans = Vec::new();
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut i = 0;
        while i < n {
            let mut j = i;
            while j < n && bytes[j] != DELIMITER as u8 {
                j += 1;
            }
            let len_str = std::str::from_utf8(&bytes[i..j]).unwrap();
            let len = len_str.parse::<usize>().unwrap();
            j += 1; // skip delimiter
            let str_val = std::str::from_utf8(&bytes[j..j + len]).unwrap().to_string();
            ans.push(str_val);
            i = j + len;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helper::map_test_case_to_string;

    #[test]
    fn test_case_1() {
        let strs = vec!["neet", "code", "love", "you"];
        let strs = map_test_case_to_string(strs);
        let encoded = Solution::encode(strs.clone());
        let decoded = Solution::decode(encoded);
        assert_eq!(strs, decoded);
    }

    #[test]
    fn test_case_2() {
        let strs = vec!["we", "say", ":", "yes"];
        let strs = map_test_case_to_string(strs);
        let encoded = Solution::encode(strs.clone());
        let decoded = Solution::decode(encoded);
        assert_eq!(strs, decoded);
    }
}
