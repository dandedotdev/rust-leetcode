// <Array, Sorting>
// Time: O(n log n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|interval| interval[0]);
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());
        for interval in intervals {
            match ans.last() {
                None => ans.push(interval),
                Some(last) if last[1] < interval[0] => ans.push(interval),
                Some(_) => {
                    ans.last_mut().unwrap()[1] = ans.last().unwrap()[1].max(interval[1]);
                },
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
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let result = Solution::merge(intervals);
        let expected = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let result = Solution::merge(intervals);
        let expected = vec![vec![1, 5]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let intervals = vec![vec![4, 7], vec![1, 4]];
        let result = Solution::merge(intervals);
        let expected = vec![vec![1, 7]];
        assert_eq!(result, expected);
    }
}
