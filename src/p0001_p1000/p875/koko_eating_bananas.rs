// <Array, Binary Search>
// Time: O(n log m), n = len(piles), m = max(piles)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut low, mut high) = (1, *piles.iter().max().unwrap());
        while low < high {
            let mid = (low + high) >> 1;
            if Self::can_eat_all(&piles, mid, h) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        high
    }

    fn can_eat_all(piles: &[i32], speed: i32, h: i32) -> bool {
        let mut hours = 0;
        for &pile in piles {
            // `f32` is not allowed here
            hours += (pile as f64 / speed as f64).ceil() as i32;
        }
        hours <= h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;
        let result = Solution::min_eating_speed(piles, h);
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 5;
        let result = Solution::min_eating_speed(piles, h);
        let expected = 30;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_123() {
        let piles = vec![1000000000];
        let h = 2;
        let result = Solution::min_eating_speed(piles, h);
        let expected = 500000000;
        assert_eq!(result, expected);
    }
}
