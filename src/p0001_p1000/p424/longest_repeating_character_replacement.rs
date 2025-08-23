// <Hash Table, String, Sliding Window>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut freq = [0; 26];
        let mut max_freq = 0;
        let mut left = 0;
        s.bytes().enumerate().fold(0, |max_len, (right, byte)| {
            let idx = (byte - b'A') as usize;
            freq[idx] += 1;
            max_freq = max_freq.max(freq[idx]);
            if (right - left + 1) - max_freq > k as usize {
                freq[(s.as_bytes()[left] - b'A') as usize] -= 1;
                left += 1;
            }
            max_len.max(right - left + 1)
        }) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "ABAB";
        let k = 2;
        let result = Solution::character_replacement(s.to_string(), k);
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let s = "AABABBA";
        let k = 1;
        let result = Solution::character_replacement(s.to_string(), k);
        let expected = 4;
        assert_eq!(result, expected);
    }
}
