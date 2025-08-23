use std::collections::HashMap;

// <HashTable>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (idx, &num) in nums.iter().enumerate() {
            let complement: i32 = target - num;
            if map.contains_key(&complement) {
                return vec![*map.get(&complement).unwrap() as i32, idx as i32];
            }
            map.insert(num, idx);
        }
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);
        let expected = vec![0, 1];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        let expected = vec![1, 2];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![3, 3];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        let expected = vec![0, 1];
        assert_eq!(result, expected);
    }
}
