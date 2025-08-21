// <Array, Binary Search, Divide and Conquer>
// Time: O(log std::cmp::min(m + n))
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };
        let (m, n) = (nums1.len(), nums2.len());
        let (mut left, mut right) = (0, m);

        while left <= right {
            let i = (left + right) >> 1;
            let j = ((m + n + 1) >> 1) - i; // `+ 1` to make sure when the amount of elements is odd, the `i` has one more element. We can easily get `max_l_1` as `last()`.

            let max_l_1 = if i == 0 { i32::MIN } else { nums1[i - 1] };
            let min_r_i = if i == m { i32::MAX } else { nums1[i] };
            let max_l_2 = if j == 0 { i32::MIN } else { nums2[j - 1] };
            let min_r_2 = if j == n { i32::MAX } else { nums2[j] };

            if max_l_1 <= min_r_2 && max_l_2 <= min_r_i {
                if (m + n) % 2 == 0 {
                    return (max_l_1.max(max_l_2) + min_r_i.min(min_r_2)) as f64 / 2.0;
                } else {
                    return max_l_1.max(max_l_2) as f64;
                }
            }

            if max_l_1 > min_r_2 {
                right = i - 1;
            } else {
                left = i + 1;
            }
        }

        0.0
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
