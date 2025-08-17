use std::cmp::Ordering;

// <Array, Binary Search (Exclusive Bounds), Matrix>
// Time: O(log (m * n))
// Space: O(m * n)

pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let flattened = matrix.iter().flatten().collect::<Vec<_>>();
        let (mut left, mut right) = (0, flattened.len());

        while left < right {
            let mid = (left + right) >> 1;

            match flattened[mid].cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Less => left = mid + 1,
                Ordering::Greater => right = mid,
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 3;
        let result = Solution::search_matrix(matrix, target);

        assert!(result);
    }

    #[test]
    fn test_case_2() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 13;
        let result = Solution::search_matrix(matrix, target);

        assert!(!result);
    }
}
