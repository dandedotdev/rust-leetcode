// <Array, Hash Table, Prefix Sum>
// Time: O(n * m)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        (left..=right).all(|num| {
            ranges
                .iter()
                .any(|range| range[0] <= num && num <= range[1])
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let ranges = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let left = 2;
        let right = 5;
        let result = Solution::is_covered(ranges, left, right);
        assert!(result);
    }

    #[test]
    fn test_case_2() {
        let ranges = vec![vec![1, 10], vec![10, 20]];
        let left = 21;
        let right = 21;
        let result = Solution::is_covered(ranges, left, right);
        assert!(!result);
    }
}
