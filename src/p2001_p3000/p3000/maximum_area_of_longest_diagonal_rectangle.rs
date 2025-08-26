// <Array>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut max_dia_sq = 0;
        let mut ans = 0;
        for d in dimensions {
            let dia_sq = d[0] * d[0] + d[1] * d[1];
            let area = d[0] * d[1];
            if dia_sq > max_dia_sq {
                max_dia_sq = dia_sq;
                ans = area;
            } else if dia_sq == max_dia_sq {
                ans = ans.max(area);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let dimensions = vec![vec![9, 3], vec![8, 6]];
        let result = Solution::area_of_max_diagonal(dimensions);
        let expected = 48;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let dimensions = vec![vec![3, 4], vec![4, 3]];
        let result = Solution::area_of_max_diagonal(dimensions);
        let expected = 12;
        assert_eq!(result, expected);
    }
}
