// <Array, Two Pointers, Dynamic Programming, Stack, Monotonic Stack>
// Time: O(n)
// Space: O(1)

// Note:
// We only care about the shorter height.

pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }

        let (mut left, mut right) = (0, height.len() - 1);
        let (mut max_left, mut max_right) = (height[left], height[right]);
        let mut result = 0;

        while left < right {
            if max_left < max_right {
                left += 1;
                max_left = max_left.max(height[left]);
                result += max_left - height[left];
            } else {
                right -= 1;
                max_right = max_right.max(height[right]);
                result += max_right - height[right];
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let result = Solution::trap(height);
        let expected = 6;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        let result = Solution::trap(height);
        let expected = 9;

        assert_eq!(result, expected);
    }
}
