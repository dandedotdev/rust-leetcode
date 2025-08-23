use std::collections::HashMap;

// <Hash Table, String, Sliding Window>
// Time: O(m), m = s2.len()
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (m, n) = (s1.len(), s2.len());
        if m > n {
            return false;
        }
        let mut memo: HashMap<u8, i32> = HashMap::new();
        for &byte in s1.as_bytes() {
            *memo.entry(byte).or_default() += 1;
        }
        let mut window = HashMap::new();
        let (mut start, mut end) = (0, 0);
        let mut valid_count = 0;
        while end < n {
            let end_char = s2.as_bytes()[end];
            if memo.contains_key(&end_char) {
                *window.entry(end_char).or_default() += 1;
                if window.get(&end_char) == memo.get(&end_char) {
                    valid_count += 1;
                }
            }
            end += 1;
            while valid_count == memo.len() {
                if end - start == m {
                    return true;
                }
                let start_char = s2.as_bytes()[start];
                if memo.contains_key(&start_char) {
                    if window.get(&start_char) == memo.get(&start_char) {
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
