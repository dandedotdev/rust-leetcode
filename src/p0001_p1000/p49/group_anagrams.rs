use std::collections::HashMap;

// <Array, Hash Table, String, Sorting>
// Time: O(n * m)
// Space: O(n * m)

pub struct Solution;

impl Solution {
    fn count_chars(s: &str) -> [i32; 26] {
        let mut freq: [i32; 26] = [0; 26];

        for &byte in s.as_bytes() {
            freq[(byte - b'a') as usize] += 1;
        }

        freq
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut memo = HashMap::new();

        for s in strs.iter() {
            let key = Self::count_chars(s);

            memo.entry(key).or_insert(Vec::new()).push(s.clone());
        }

        memo.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_helper::normalize_result;

    use super::*;

    #[test]
    fn test_case_1() {
        let s = ["eat", "tea", "tan", "ate", "nat", "bat"]
            .map(String::from)
            .to_vec();
        let result = Solution::group_anagrams(s);
        let expected = vec![
            ["eat", "tea", "ate"].map(String::from).to_vec(),
            ["tan", "nat"].map(String::from).to_vec(),
            ["bat"].map(String::from).to_vec(),
        ];

        assert_eq!(normalize_result(result), normalize_result(expected));
    }

    #[test]
    fn test_case_2() {
        let s = [""].map(String::from).to_vec();
        let result = Solution::group_anagrams(s);
        let expected = vec![[""].map(String::from).to_vec()];

        assert_eq!(normalize_result(result), normalize_result(expected));
    }

    #[test]
    fn test_case_3() {
        let s = ["a"].map(String::from).to_vec();
        let result = Solution::group_anagrams(s);
        let expected = vec![["a"].map(String::from).to_vec()];

        assert_eq!(normalize_result(result), normalize_result(expected));
    }
}
