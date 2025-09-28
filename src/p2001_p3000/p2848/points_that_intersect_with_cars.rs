// <Array, Hash Table, Prefix Sum>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        let mut diff = vec![0; 102];
        nums.into_iter().for_each(|v| {
            diff[v[0] as usize] += 1;
            diff[(v[1] + 1) as usize] -= 1;
        });
        diff.into_iter()
            .fold((0, 0), |(prefix_sum, ans), x| {
                let prefix_sum = prefix_sum + x;
                (prefix_sum, ans + (prefix_sum > 0) as i32)
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![vec![3, 6], vec![1, 5], vec![4, 7]];
        let result = Solution::number_of_points(nums);
        let expected = 7;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![vec![1, 3], vec![5, 8]];
        let result = Solution::number_of_points(nums);
        let expected = 7;
        assert_eq!(result, expected);
    }
}
