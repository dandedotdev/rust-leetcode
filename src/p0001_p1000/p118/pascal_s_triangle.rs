// <Array, Dynamic Programming>
// Time: O(n^2)
// Space: O(n^2)

pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![vec![1]];
        for i in 1..num_rows as usize {
            let mut row = vec![1; i + 1];
            for (j, item) in row.iter_mut().enumerate().take(i).skip(1) {
                *item = ans[i - 1][j - 1] + ans[i - 1][j];
            }
            ans.push(row);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let num_rows = 5;
        let result = Solution::generate(num_rows);
        let expected = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let num_rows = 1;
        let result = Solution::generate(num_rows);
        let expected = vec![vec![1]];
        assert_eq!(result, expected);
    }
}
