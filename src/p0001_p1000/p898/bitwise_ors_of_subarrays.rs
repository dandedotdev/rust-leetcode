use std::collections::HashSet;

// <Array, Dynamic Programming, Bit Manipulation>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn subarray_bitwise_ors(arr: Vec<i32>) -> i32 {
        let mut current_or: Vec<i32> = vec![];
        let mut result_or = HashSet::new();

        for num in arr {
            for cur in &mut current_or {
                *cur |= num;
            }

            current_or.push(num);
            current_or.dedup();
            result_or.extend(current_or.iter().copied());
        }

        result_or.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let arr = vec![0];
        let result = Solution::subarray_bitwise_ors(arr);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_case_2() {
        let arr = vec![1, 1, 2];
        let result = Solution::subarray_bitwise_ors(arr);

        assert_eq!(result, 3);
    }

    #[test]
    fn test_case_3() {
        let arr = vec![1, 2, 4];
        let result = Solution::subarray_bitwise_ors(arr);

        assert_eq!(result, 6);
    }

    #[test]
    fn test_case_50() {
        let arr = vec![
            243, 202, 1002, 412, 422, 823, 932, 558, 868, 517, 938, 856, 214, 856, 938, 5, 739,
            950, 518, 448,
        ];
        let result = Solution::subarray_bitwise_ors(arr);

        assert_eq!(result, 38);
    }
}
