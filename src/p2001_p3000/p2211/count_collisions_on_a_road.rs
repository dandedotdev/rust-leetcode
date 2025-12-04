// <Simulation, Stack, String>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let bytes = directions.as_bytes();
        let left_bound = bytes.iter().position(|&b| b != b'L');
        let right_bound = bytes.iter().rposition(|&b| b != b'R');
        if left_bound.is_none() || right_bound.is_none() {
            return 0;
        }
        let left_bound = left_bound.unwrap();
        let right_bound = right_bound.unwrap();
        // No collisions if the left bound is greater than the right bound
        if left_bound > right_bound {
            return 0;
        }
        bytes[left_bound..=right_bound]
            .iter()
            .filter(|&&b| b != b'S')
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let directions = "RLRSLL".to_string();
        let result = Solution::count_collisions(directions);
        let expected = 5;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let directions = "LLRR".to_string();
        let result = Solution::count_collisions(directions);
        let expected = 0;
        assert_eq!(result, expected);
    }
}
