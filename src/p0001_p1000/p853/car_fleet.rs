// <Array, Sorting, Stack, Monotonic Stack>
// Time: O(n log n)
// Space: O(log n) <auxiliary space for sorting>

pub struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars = position
            .into_iter()
            .zip(speed)
            .map(|(pos, speed)| (pos, (target as f32 - pos as f32) / speed as f32))
            .collect::<Vec<(i32, f32)>>();

        cars.sort_unstable_by_key(|(pos, _)| std::cmp::Reverse(*pos));

        let mut fleets = Vec::new();

        for (_, arrival_time) in cars {
            if fleets.last().is_none_or(|top| *top < arrival_time) {
                fleets.push(arrival_time);
            }
        }

        fleets.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let target = 12;
        let position = vec![10, 8, 0, 5, 3];
        let speed = vec![2, 4, 1, 1, 3];
        let expected = 3;

        assert_eq!(Solution::car_fleet(target, position, speed), expected);
    }

    #[test]
    fn test_case_2() {
        let target = 10;
        let position = vec![3];
        let speed = vec![3];
        let expected = 1;

        assert_eq!(Solution::car_fleet(target, position, speed), expected);
    }

    #[test]
    fn test_case_3() {
        let target = 100;
        let position = vec![0, 2, 4];
        let speed = vec![4, 2, 1];
        let expected = 1;

        assert_eq!(Solution::car_fleet(target, position, speed), expected);
    }
}
