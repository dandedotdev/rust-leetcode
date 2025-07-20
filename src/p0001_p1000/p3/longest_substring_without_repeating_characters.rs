// <Hash Table, String, Sliding Window>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_char_index = [0; 128]; // ASCII table
        let mut start = 0;
        let mut max_length = 0;

        for (end, char) in s.chars().enumerate() {
            start = start.max(last_char_index[char as usize]);
            max_length = max_length.max(end - start + 1);
            last_char_index[char as usize] = end + 1;
        }

        max_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "abcabcbb".to_string();
        let expected = 3;
        let result = Solution::length_of_longest_substring(s);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let s = "bbbbb".to_string();
        let expected = 1;
        let result = Solution::length_of_longest_substring(s);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let s = "pwwkew".to_string();
        let expected = 3;
        let result = Solution::length_of_longest_substring(s);

        assert_eq!(result, expected);
    }
}
