// <Matrix>
// Time: O(m * n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();

        if m == 0 {
            return;
        }

        let n = matrix[0].len();

        if n == 0 {
            return;
        }

        let mut is_zero_in_the_first_row = false;
        let mut is_zero_in_the_first_col = false;

        for row in matrix.iter() {
            if row[0] == 0 {
                is_zero_in_the_first_col = true;
                break;
            }
        }

        for &cell in matrix[0].iter() {
            if cell == 0 {
                is_zero_in_the_first_row = true;
                break;
            }
        }

        for row in 1..m {
            for col in 1..n {
                if matrix[row][col] == 0 {
                    matrix[row][0] = 0;
                    matrix[0][col] = 0;
                }
            }
        }

        for row in matrix.iter_mut().skip(1) {
            if row[0] == 0 {
                for cell in row.iter_mut().skip(1) {
                    *cell = 0;
                }
            }
        }

        for col in 1..n {
            if matrix[0][col] == 0 {
                for row in matrix.iter_mut().skip(1) {
                    row[col] = 0;
                }
            }
        }

        if is_zero_in_the_first_row {
            for cell in matrix[0].iter_mut() {
                *cell = 0;
            }
        }

        if is_zero_in_the_first_col {
            for row in matrix.iter_mut() {
                row[0] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut matrix);

        assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
    }

    #[test]
    fn test_case_2() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut matrix);

        assert_eq!(matrix, vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![
            0, 3, 1, 0
        ]]);
    }
}
