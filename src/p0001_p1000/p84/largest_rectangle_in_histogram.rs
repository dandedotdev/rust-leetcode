// <Array, Monotonic Stack, Stack>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut ans = 0;
        let mut stk = vec![(0, heights[0])];
        for (i, &height) in heights.iter().enumerate() {
            while let Some(&(_, top_height)) = stk.last() {
                if height < top_height {
                    let (_, h) = stk.pop().unwrap();
                    let left = stk.last().map_or(0, |&(idx, _)| idx + 1);
                    ans = ans.max(h * (i - left) as i32);
                } else {
                    break;
                }
            }
            stk.push((i, height));
        }
        while let Some((_, height)) = stk.pop() {
            let left = stk.last().map_or(0, |&(idx, _)| idx + 1);
            ans = ans.max(height * (n - left) as i32);
        }
        ans
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
