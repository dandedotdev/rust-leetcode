// <Array, Math, Stack>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();

        tokens.into_iter().for_each(|char| match char.as_str() {
            "+" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            }
            "-" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            }
            "*" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a * b);
            }
            "/" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a / b);
            }
            _ => {
                stack.push(char.parse().unwrap());
            }
        });

        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_helper::map_test_case_to_string;

    use super::*;

    #[test]
    fn test_case_1() {
        let tokens = vec!["2", "1", "+", "3", "*"];
        let tokens = map_test_case_to_string(tokens);
        let result = Solution::eval_rpn(tokens);

        assert_eq!(result, 9);
    }

    #[test]
    fn test_case_2() {
        let tokens = vec!["4", "13", "5", "/", "+"];
        let tokens = map_test_case_to_string(tokens);
        let result = Solution::eval_rpn(tokens);

        assert_eq!(result, 6);
    }

    #[test]
    fn test_case_3() {
        let tokens = vec![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ];
        let tokens = map_test_case_to_string(tokens);
        let result = Solution::eval_rpn(tokens);

        assert_eq!(result, 22);
    }
}
