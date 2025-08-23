// <Array, String>
// Time: O(n * m)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        words
            .iter()
            .enumerate()
            .filter(|(_, word)| word.contains(x))
            .map(|(idx, _)| idx as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let words = ["leet", "code"].iter().map(|s| s.to_string()).collect();
        let x = 'e';
        let result = Solution::find_words_containing(words, x);
        let expected = vec![0, 1];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let words = ["abc", "bcd", "aaaa", "cbc"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let x = 'a';
        let result = Solution::find_words_containing(words, x);
        let expected = vec![0, 2];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let words = ["abc", "bcd", "aaaa", "cbc"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let x = 'z';
        let result = Solution::find_words_containing(words, x);
        let expected = Vec::new();
        assert_eq!(result, expected);
    }
}
