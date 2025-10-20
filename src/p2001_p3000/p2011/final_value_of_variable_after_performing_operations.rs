// <Array, Simulation, String>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations.iter().fold(0, |acc, op| {
            match op.as_str() {
                "++X" | "X++" => acc + 1,
                "--X" | "X--" => acc - 1,
                _ => acc,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helper::map_test_case_to_string;

    #[test]
    fn test_case_1() {
        let operations = vec!["--X", "X++", "X++"];
        let operations = map_test_case_to_string(operations);
        let result = Solution::final_value_after_operations(operations);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let operations = vec!["++X", "++X", "X++"];
        let operations = map_test_case_to_string(operations);
        let result = Solution::final_value_after_operations(operations);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let operations = vec!["X++", "++X", "--X", "X--"];
        let operations = map_test_case_to_string(operations);
        let result = Solution::final_value_after_operations(operations);
        let expected = 0;
        assert_eq!(result, expected);
    }
}
