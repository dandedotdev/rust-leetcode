// <Math>
// Time: O(1)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        match *nums.as_slice() {
            [a, b, c] if !(a + b > c && a + c > b && b + c > a) => "none",
            [a, b, c] if a == b && b == c => "equilateral",
            [a, b, c] if a == b || b == c || a == c => "isosceles",
            _ => "scalene",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![3, 3, 3];
        let result = Solution::triangle_type(nums);
        let expected = "equilateral";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![3, 4, 5];
        let result = Solution::triangle_type(nums);
        let expected = "scalene";
        assert_eq!(result, expected);
    }
}
