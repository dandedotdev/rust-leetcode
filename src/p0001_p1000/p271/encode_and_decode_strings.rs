// <Array, Two Pointers>
// Time: O(n)
// SPace: O(m)

const DELIMITER: char = ';';

pub struct Solution;

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut result = String::default();

        for s in strs {
            result.push_str(&format!("{}{}{}", s.len(), DELIMITER, s));
        }

        result
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut result = Vec::new();
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

            result.push(str_val);
            i = j + len;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let strs: Vec<String> = vec!["neet", "code", "love", "you"]
            .into_iter()
            .map(String::from)
            .collect();
        let encoded = Solution::encode(strs.clone());
        let decoded = Solution::decode(encoded);

        assert_eq!(strs, decoded);
    }

    #[test]
    fn test_case_2() {
        let strs: Vec<String> = vec!["we", "say", ":", "yes"]
            .into_iter()
            .map(String::from)
            .collect();
        let encoded = Solution::encode(strs.clone());
        let decoded = Solution::decode(encoded);

        assert_eq!(strs, decoded);
    }
}
