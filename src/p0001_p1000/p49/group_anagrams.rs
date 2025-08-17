use std::collections::HashMap;

// <Array, Hash Table, String, Sorting>
// Time: O(n * m)
// Space: O(n * m)

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut memo = HashMap::new();

        for s in strs.iter() {
            let key = Self::count_chars(s);

            memo.entry(key).or_insert(Vec::new()).push(s.clone());
        }

        memo.into_values().collect()
    }

    fn count_chars(s: &str) -> [i32; 26] {
        let mut freq: [i32; 26] = [0; 26];

        for &byte in s.as_bytes() {
            freq[(byte - b'a') as usize] += 1;
        }

        freq
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_helper::{map_test_case_to_string, normalize_result};

    use super::*;

    #[test]
    fn test_case_1() {
        let s = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
        let s = map_test_case_to_string(s);
        let result = Solution::group_anagrams(s);
        let expected = vec![
            map_test_case_to_string(vec!["eat", "tea", "ate"]),
            map_test_case_to_string(vec!["tan", "nat"]),
            map_test_case_to_string(vec!["bat"]),
        ];

        assert_eq!(normalize_result(result), normalize_result(expected));
    }

    #[test]
    fn test_case_2() {
        let s = vec![""];
        let s = map_test_case_to_string(s);
        let result = Solution::group_anagrams(s);
        let expected = vec![map_test_case_to_string(vec![""])];

        assert_eq!(normalize_result(result), normalize_result(expected));
    }

    #[test]
    fn test_case_3() {
        let s = vec!["a"];
        let s = map_test_case_to_string(s);
        let result = Solution::group_anagrams(s);
        let expected = vec![map_test_case_to_string(vec!["a"])];

        assert_eq!(normalize_result(result), normalize_result(expected));
    }
}
