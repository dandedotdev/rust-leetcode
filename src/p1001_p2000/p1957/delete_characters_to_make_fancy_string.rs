// <String>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut prev_char = '\0';
        let mut consecutive_count = 0;
        let mut result = String::with_capacity(s.len());

        for c in s.chars() {
            if c == prev_char {
                if consecutive_count < 2 {
                    consecutive_count += 1;
                } else {
                    continue;
                }
            } else {
                prev_char = c;
                consecutive_count = 1;
            }

            result.push(c);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "leeetcode".to_string();
        let result = Solution::make_fancy_string(s);
        let expected = "leetcode".to_string();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let s = "aaabaaaa".to_string();
        let result = Solution::make_fancy_string(s);
        let expected = "aabaa".to_string();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let s = "aab".to_string();
        let result = Solution::make_fancy_string(s);
        let expected = "aab".to_string();

        assert_eq!(result, expected);
    }
}
