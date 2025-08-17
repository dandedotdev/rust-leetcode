// <Array, Monotonic Stack, Stack>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut stack: Vec<usize> = Vec::new();
        let mut i = 0;

        while i < heights.len() {
            if stack.is_empty() || heights[i] >= heights[stack[stack.len() - 1]] {
                stack.push(i);
                i += 1;
            } else {
                let top = stack.pop().unwrap();
                let area = heights[top]
                    * if stack.is_empty() {
                        i as i32
                    } else {
                        // i - 1, cur_idx is not included
                        (i - 1 - stack[stack.len() - 1]) as i32
                    };
                max_area = max_area.max(area);
            }
        }

        while let Some(top) = stack.pop() {
            let area = heights[top]
                * if stack.is_empty() {
                    i as i32
                } else {
                    // i - 1, cur_idx is not included
                    (i - 1 - stack[stack.len() - 1]) as i32
                };
            max_area = max_area.max(area);
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
