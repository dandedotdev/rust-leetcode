// <Array>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let n = intervals.len();
        let mut idx = 0;
        let mut ans = Vec::with_capacity(n + 1); // + 1 for the new interval
        while idx < n && intervals[idx][1] < new_interval[0] {
            ans.push(vec![intervals[idx][0], intervals[idx][1]]);
            idx += 1;
        }
        while idx < n && intervals[idx][0] <= new_interval[1] {
            new_interval[0] = new_interval[0].min(intervals[idx][0]);
            new_interval[1] = new_interval[1].max(intervals[idx][1]);
            idx += 1;
        }
        ans.push(new_interval);
        while idx < n {
            ans.push(vec![intervals[idx][0], intervals[idx][1]]);
            idx += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let result = Solution::insert(intervals, new_interval);
        let expected = vec![vec![1, 5], vec![6, 9]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let new_interval = vec![4, 8];
        let result = Solution::insert(intervals, new_interval);
        let expected = vec![vec![1, 2], vec![3, 10], vec![12, 16]];
        assert_eq!(result, expected);
    }
}
