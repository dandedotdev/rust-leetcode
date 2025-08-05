// <Hash Table, String, Sliding Window>
// Time: O(m + n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let m = s.len();
        let n = t.len();

        if m < n {
            return String::default();
        }

        let (bytes_s, bytes_t) = (s.as_bytes(), t.as_bytes());
        let mut memo = [0; 128];

        for &byte in bytes_t {
            memo[byte as usize] += 1;
        }

        let mut count = 0;
        let mut start = 0;
        let mut start_idx = 0;
        let mut min_len = usize::MAX;

        for end in 0..m {
            if memo[bytes_s[end] as usize] > 0 {
                count += 1;
            }

            memo[bytes_s[end] as usize] -= 1;

            while count == n {
                if end - start + 1 < min_len {
                    min_len = end - start + 1;
                    start_idx = start;
                }

                memo[bytes_s[start] as usize] += 1;

                if memo[bytes_s[start] as usize] > 0 {
                    count -= 1;
                }

                start += 1;
            }
        }

        if min_len == usize::MAX {
            String::default()
        } else {
            s[start_idx..start_idx + min_len].to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        let result = Solution::min_window(s, t);
        let expected = "BANC".to_string();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let s = "a".to_string();
        let t = "a".to_string();
        let result = Solution::min_window(s, t);
        let expected = "a".to_string();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let s = "a".to_string();
        let t = "aa".to_string();
        let result = Solution::min_window(s, t);
        let expected = "".to_string();

        assert_eq!(result, expected);
    }
}
