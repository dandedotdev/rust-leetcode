// <Array, Binary Search, Divide and Conquer>
// Time: O(m + n)
// Space: O(m + n)

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged: Vec<i32> = Vec::new();
        let (mut i, mut j) = (0, 0);
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                merged.push(nums1[i]);
                i += 1;
            } else {
                merged.push(nums2[j]);
                j += 1;
            }
        }
        while i < nums1.len() {
            merged.push(nums1[i]);
            i += 1;
        }
        while j < nums2.len() {
            merged.push(nums2[j]);
            j += 1;
        }
        let mid = merged.len() >> 1;
        if merged.len().is_multiple_of(2) {
            return (merged[mid - 1] + merged[mid]) as f64 / 2.0;
        }
        merged[mid] as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let result = Solution::find_median_sorted_arrays(nums1, nums2);
        let expected = 2.0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        let result = Solution::find_median_sorted_arrays(nums1, nums2);
        let expected = 2.5;
        assert_eq!(result, expected);
    }
}
