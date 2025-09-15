// <Hash Table, String>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut broken = [false; 26];
        broken_letters.bytes().for_each(|b| broken[(b - b'a') as usize] = true);
        text.split_whitespace()
            .filter(|word| !word.bytes().any(|b| broken[(b - b'a') as usize]))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let text = "hello world".to_string();
        let broken_letters = "ad".to_string();
        let result = Solution::can_be_typed_words(text, broken_letters);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let text = "leet code".to_string();
        let broken_letters = "lt".to_string();
        let result = Solution::can_be_typed_words(text, broken_letters);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let text = "leet code".to_string();
        let broken_letters = "e".to_string();
        let result = Solution::can_be_typed_words(text, broken_letters);
        let expected = 0;
        assert_eq!(result, expected);
    }
}
