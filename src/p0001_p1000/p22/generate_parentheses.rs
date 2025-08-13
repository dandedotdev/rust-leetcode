// <Backtracking, Dynamic Programming, String>
// Time: O(4 ** n / (n ** 1 / 2))
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut s = String::default();

        Self::backtrack(&mut result, &mut s, 0, 0, n);

        result
    }

    fn backtrack(result: &mut Vec<String>, cur_string: &mut String, open: i32, close: i32, n: i32) {
        if close == n && open == n {
            result.push(cur_string.clone());
            return;
        }

        if open < n {
            cur_string.push('(');
            Self::backtrack(result, cur_string, open + 1, close, n);
            cur_string.pop();
        }

        if close < open {
            cur_string.push(')');
            Self::backtrack(result, cur_string, open, close + 1, n);
            cur_string.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_helper::map_test_case_to_string;

    use super::*;

    #[test]
    fn test_case_1() {
        let n = 3;
        let expected =
            map_test_case_to_string(vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
        assert_eq!(Solution::generate_parenthesis(n), expected);
    }

    #[test]
    fn test_case_2() {
        let n = 1;
        let expected = map_test_case_to_string(vec!["()"]);
        assert_eq!(Solution::generate_parenthesis(n), expected);
    }
}
