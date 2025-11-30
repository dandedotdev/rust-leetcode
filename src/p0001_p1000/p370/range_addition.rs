// <Array, Prefix Sum>
// Time: O(n + k), where n = length, k = updates.len()
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![0; length as usize];
        {
            updates.iter().for_each(|update| {
                if let [start, end, value] = update.as_slice() {
                    ans[*start as usize] += *value;
                    if *end + 1 < length {
                        ans[(*end + 1) as usize] -= *value;
                    }
                }
            });
            ans.iter_mut().fold(0, |prefix_sum: i32, x| {
                let cur = prefix_sum + *x;
                *x = cur;
                cur
            });
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let length = 5;
        let updates = vec![vec![1, 3, 2], vec![2, 4, 3], vec![0, 2, -2]];
        let result = Solution::get_modified_array(length, updates);
        let expected = vec![-2, 0, 3, 5, 3];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let length = 10;
        let updates = vec![vec![2, 4, 6], vec![5, 6, 8], vec![1, 9, -4]];
        let result = Solution::get_modified_array(length, updates);
        let expected = vec![0, -4, 2, 2, 2, 4, 4, -4, -4, -4];
        assert_eq!(result, expected);
    }
}
