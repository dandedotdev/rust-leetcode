// <Stack, String>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len() / 2);

        for &b in s.as_bytes() {
            match b {
                b'(' | b'[' | b'{' => stack.push(b),
                b')' => {
                    if stack.pop() != Some(b'(') {
                        return false;
                    }
                }
                b']' => {
                    if stack.pop() != Some(b'[') {
                        return false;
                    }
                }
                b'}' => {
                    if stack.pop() != Some(b'{') {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "()".to_string();
        let result = Solution::is_valid(s);

        assert!(result);
    }

    #[test]
    fn test_case_2() {
        let s = "()[]{}".to_string();
        let result = Solution::is_valid(s);

        assert!(result);
    }

    #[test]
    fn test_case_3() {
        let s = "(]".to_string();
        let result = Solution::is_valid(s);

        assert!(!result);
    }

    #[test]
    fn test_case_4() {
        let s = "([])".to_string();
        let result = Solution::is_valid(s);

        assert!(result);
    }

    #[test]
    fn test_case_5() {
        let s = "([)]".to_string();
        let result = Solution::is_valid(s);

        assert!(!result);
    }
}
