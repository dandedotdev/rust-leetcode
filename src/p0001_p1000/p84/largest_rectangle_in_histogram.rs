// <Array, Monotonic Stack, Stack>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut max_area = 0;
        let mut stack = vec![(0, heights[0])];

        for (i, &height) in heights.iter().enumerate() {
            while let Some(&(_, top_height)) = stack.last() {
                if height < top_height {
                    let (_, h) = stack.pop().unwrap();
                    let left = stack.last().map_or(0, |&(idx, _)| idx + 1);
                    max_area = max_area.max(h * (i - left) as i32);
                } else {
                    break;
                }
            }
            stack.push((i, height));
        }

        while let Some((_, height)) = stack.pop() {
            let left = stack.last().map_or(0, |&(idx, _)| idx + 1);
            max_area = max_area.max(height * (n - left) as i32);
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        let result = Solution::largest_rectangle_area(heights);
        let expected = 10;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let heights = vec![2, 4];
        let result = Solution::largest_rectangle_area(heights);
        let expected = 4;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_69() {
        let heights = vec![1, 2, 2];
        let result = Solution::largest_rectangle_area(heights);
        let expected = 4;

        assert_eq!(result, expected);
    }
}
