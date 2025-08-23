// <Array, Segment Tree>
// Time: O(n log n), n = m
// Space: O(n)

// Note: Faster than 3477. Fruits Into Baskets II

pub struct Solution;

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let n = baskets.len();
        let mut leaf_idx = 1;
        while leaf_idx <= n {
            leaf_idx <<= 1;
        }
        let mut seg = vec![0; leaf_idx << 1];
        seg[leaf_idx..(leaf_idx + n)].copy_from_slice(&baskets[..n]);
        for i in (1..leaf_idx).rev() {
            seg[i] = seg[2 * i].max(seg[2 * i + 1]);
        }
        let mut count = 0;
        for &fruit in &fruits {
            let mut idx = 1;
            if seg[idx] < fruit {
                count += 1;
                continue;
            }
            while idx < leaf_idx {
                if seg[2 * idx] >= fruit {
                    idx *= 2;
                } else {
                    idx = 2 * idx + 1;
                }
            }
            seg[idx] = -1;
            // Update the segment tree
            let mut temp = idx;
            while temp > 1 {
                temp >>= 1;
                seg[temp] = seg[2 * temp].max(seg[2 * temp + 1]);
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let fruits = vec![4, 2, 5];
        let baskets = vec![3, 5, 4];
        let result = Solution::num_of_unplaced_fruits(fruits, baskets);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let fruits = vec![3, 6, 1];
        let baskets = vec![6, 4, 7];
        let result = Solution::num_of_unplaced_fruits(fruits, baskets);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_737() {
        // O(n ** 2) TLE!
    }
}
