// <Array, Geometry, Math>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        points
            .windows(2)
            .map(|w| (w[0][0] - w[1][0]).abs().max((w[0][1] - w[1][1]).abs()))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let points = vec![vec![1, 1], vec![3, 4], vec![-1, 0]];
        let result = Solution::min_time_to_visit_all_points(points);
        let expected = 7;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let points = vec![vec![3, 2], vec![-2, 2]];
        let result = Solution::min_time_to_visit_all_points(points);
        let expected = 5;
        assert_eq!(result, expected);
    }
}
