// <Hash Table, String>
// Time: O(m), m = s2.len()
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let n = s1.len();
        let m = s2.len();

        if n > m {
            return false;
        }

        let (mut dict_s1, mut dict_s2) = ([0; 26], [0; 26]);
        let (bytes_s1, bytes_s2) = (s1.as_bytes(), s2.as_bytes());

        for i in 0..n {
            dict_s1[(bytes_s1[i] - b'a') as usize] += 1;
            dict_s2[(bytes_s2[i] - b'a') as usize] += 1;
        }

        for i in n..m {
            if dict_s1 == dict_s2 {
                return true;
            }

            dict_s2[(bytes_s2[i] - b'a') as usize] += 1;
            dict_s2[(bytes_s2[i - n] - b'a') as usize] -= 1;
        }

        dict_s1 == dict_s2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s1 = "ab".to_string();
        let s2 = "eidbaooo".to_string();
        let result = Solution::check_inclusion(s1, s2);

        assert!(result);
    }

    #[test]
    fn test_case_2() {
        let s1 = "ab".to_string();
        let s2 = "eidboaoo".to_string();
        let result = Solution::check_inclusion(s1, s2);

        assert!(!result);
    }
}
