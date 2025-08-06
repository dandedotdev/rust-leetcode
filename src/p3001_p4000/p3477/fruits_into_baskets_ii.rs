// <Array, Simulation>
// Time: O(n ** 2)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, mut baskets: Vec<i32>) -> i32 {
        let mut alloted_count = 0;

        for &fruit in fruits.iter() {
            if let Some(basket) = baskets.iter_mut().find(|b| **b >= fruit) {
                *basket = -1; // mark as placed
                alloted_count += 1;
            }
        }

        (fruits.len() - alloted_count) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let fruits = vec![4, 2, 5];
        let baskets = vec![3, 5, 4];
        let result = Solution::num_of_unplaced_fruits(fruits, baskets);
        let expected = 1;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let fruits = vec![3, 6, 1];
        let baskets = vec![6, 4, 7];
        let result = Solution::num_of_unplaced_fruits(fruits, baskets);
        let expected = 0;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_425() {
        let fruits = vec![8, 5];
        let baskets = vec![1, 8];
        let result = Solution::num_of_unplaced_fruits(fruits, baskets);
        let expected = 1;

        assert_eq!(result, expected);
    }
}
