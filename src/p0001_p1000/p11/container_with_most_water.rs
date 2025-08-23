// <Array, Two Pointers>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n < 2 {
            return 0;
        }
        let mut ans = 0;
        let (mut left, mut right) = (0, n - 1);
        while left < right {
            let area = (right - left) as i32 * height[left].min(height[right]);
            ans = ans.max(area);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
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
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = Solution::max_area(height);
        let expected = 49;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let height = vec![1, 1];
        let result = Solution::max_area(height);
        let expected = 1;
        assert_eq!(result, expected);
    }
}
