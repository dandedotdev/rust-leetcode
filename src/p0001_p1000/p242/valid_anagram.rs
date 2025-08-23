use std::collections::HashMap;

// <Hash Table, String, Sorting>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut memo: HashMap<u8, i32> = HashMap::new();
        for &byte in s.as_bytes() {
            *memo.entry(byte).or_default() += 1;
        }
        for &byte in t.as_bytes() {
            memo.entry(byte).and_modify(|count| *count -= 1);
        }
        memo.into_values().all(|v| v == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        let result = Solution::is_anagram(s, t);
        assert!(result);
    }

    #[test]
    fn test_case_2() {
        let s = "rat".to_string();
        let t = "car".to_string();
        let result = Solution::is_anagram(s, t);
        assert!(!result);
    }
}
