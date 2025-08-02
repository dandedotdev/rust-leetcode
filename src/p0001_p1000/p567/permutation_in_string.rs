use std::collections::HashMap;

// <Hash Table, String, Sliding Window>
// Time: O(m), m = s2.len()
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let n = s1.len();
        let m = s2.len();

        if n > m {
            return false;
        }

        let mut dict = HashMap::new();

        for &byte in s1.as_bytes() {
            *dict.entry(byte).or_insert(0) += 1;
        }

        let mut window = HashMap::new();
        let (mut start, mut end) = (0, 0);
        let mut valid_count = 0;

        while end < m {
            let end_char = s2.as_bytes()[end];

            if dict.contains_key(&end_char) {
                *window.entry(end_char).or_insert(0) += 1;

                if window.get(&end_char) == dict.get(&end_char) {
                    valid_count += 1;
                }
            }

            end += 1;

            while valid_count == dict.len() {
                if end - start == n {
                    return true;
                }

                let start_char = s2.as_bytes()[start];

                if dict.contains_key(&start_char) {
                    if window.get(&start_char) == dict.get(&start_char) {
                        valid_count -= 1;
                    }

                    *window.get_mut(&start_char).unwrap() -= 1;
                }

                start += 1;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s1 = "ab".to_string();
        let s2 = "eidbaooo".to_string();
        let result = Solution::check_inclusion(s1, s2);

        assert!(result);
    }

    #[test]
    fn test_case_2() {
        let s1 = "ab".to_string();
        let s2 = "eidboaoo".to_string();
        let result = Solution::check_inclusion(s1, s2);

        assert!(!result);
    }
}
