use std::collections::HashMap;

// <Array, Hash Table, String, Sorting>
// Time: O(n * m)
// Space: O(n * m)

pub struct Solution;

impl Solution {
    fn count_chars(s: &str) -> [i32; 26] {
        let mut counts = [0; 26];

        for c in s.chars() {
            counts[c as usize - 'a' as usize] += 1;
        }

        counts
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();

        for s in strs.iter() {
            let key = Self::count_chars(s);

            map.entry(key).or_insert(Vec::new()).push(s.clone());
        }

        map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::normalize_result;

    use super::*;

    #[test]
    fn test_case_1() {
        let strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
            .map(String::from)
            .to_vec();
        let result = Solution::group_anagrams(strs);
        let expected = vec![
            ["eat", "tea", "ate"].map(String::from).to_vec(),
            ["tan", "nat"].map(String::from).to_vec(),
            ["bat"].map(String::from).to_vec(),
        ];

        assert_eq!(normalize_result(result), normalize_result(expected));
    }

    #[test]
    fn test_case_2() {
        let strs = [""].map(String::from).to_vec();
        let result = Solution::group_anagrams(strs);
        let expected = vec![[""].map(String::from).to_vec()];

        assert_eq!(normalize_result(result), normalize_result(expected));
    }

    #[test]
    fn test_case_3() {
        let strs = ["a"].map(String::from).to_vec();
        let result = Solution::group_anagrams(strs);
        let expected = vec![["a"].map(String::from).to_vec()];

        assert_eq!(normalize_result(result), normalize_result(expected));
    }
}
