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

        let mut max = 0;
        let (mut left, mut right) = (0, n - 1);

        while left < right {
            let area = (right - left) as i32 * height[left].min(height[right]);

            max = max.max(area);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let test_case = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];

        assert_eq!(Solution::max_area(test_case), 49);
    }

    #[test]
    fn test_case_2() {
        let test_case = vec![1, 1];

        assert_eq!(Solution::max_area(test_case), 1);
    }
}
