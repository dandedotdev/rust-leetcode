// <Array, Hash Table, Matrix>
// Time: O(1)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row = [0u16; 9];
        let mut col = [0u16; 9];
        let mut boxes = [0u16; 9];
        for (r, row_vec) in board.iter().enumerate() {
            for (c, &char) in row_vec.iter().enumerate() {
                if let Some(d) = char.to_digit(10) {
                    let bit = 1 << d;
                    let box_idx = (r / 3) * 3 + (c / 3);
                    if (row[r] & bit) != 0 || (col[c] & bit) != 0 || (boxes[box_idx] & bit) != 0 {
                        return false;
                    }
                    row[r] |= bit;
                    col[c] |= bit;
                    boxes[box_idx] |= bit;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let result = Solution::is_valid_sudoku(board);
        assert!(result);
    }

    #[test]
    fn test_case_2() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let result = Solution::is_valid_sudoku(board);
        assert!(!result);
    }
}
