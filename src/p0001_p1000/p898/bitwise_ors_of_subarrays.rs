use std::collections::HashSet;

// <Array, Dynamic Programming, Bit Manipulation>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn subarray_bitwise_ors(arr: Vec<i32>) -> i32 {
        let mut cur_or: Vec<i32> = Vec::new();
        let mut ans_or = HashSet::new();
        for num in arr {
            for cur in &mut cur_or {
                *cur |= num;
            }
            cur_or.push(num);
            cur_or.dedup();
            ans_or.extend(cur_or.iter().copied());
        }
        ans_or.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let arr = vec![0];
        let result = Solution::subarray_bitwise_ors(arr);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let arr = vec![1, 1, 2];
        let result = Solution::subarray_bitwise_ors(arr);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let arr = vec![1, 2, 4];
        let result = Solution::subarray_bitwise_ors(arr);
        let expected = 6;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_50() {
        let arr = vec![
            243, 202, 1002, 412, 422, 823, 932, 558, 868, 517, 938, 856, 214, 856, 938, 5, 739,
            950, 518, 448,
        ];
        let result = Solution::subarray_bitwise_ors(arr);
        let expected = 38;
        assert_eq!(result, expected);
    }
}
