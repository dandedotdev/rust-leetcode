use std::cmp::Ordering;

// <Array, Counting, Prefix Sum>
// Time: O(n + m), n is the number of logs, m is the duration
// Space: O(1)

pub struct Solution;

impl Solution {
    const DURATION: usize = 2050 - 1950 + 1;
    const YEAR_START: usize = 1950;

    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut diff = vec![0; Self::DURATION];
        logs.into_iter().for_each(|v| {
            diff[v[0] as usize - Self::YEAR_START] += 1;
            diff[v[1] as usize - Self::YEAR_START] -= 1;
        });
        diff.into_iter()
            .enumerate()
            .fold(
                (Self::YEAR_START as i32, 0, 0),
                |(ans, population, max_population), (idx, val)| {
                    let new_population = population + val;
                    match new_population.cmp(&max_population) {
                        Ordering::Greater =>
                            (
                                idx as i32 + Self::YEAR_START as i32,
                                new_population,
                                new_population,
                            ),
                        _ => (ans, new_population, max_population),
                    }
                },
            )
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let logs = vec![vec![1993, 1999], vec![2000, 2010]];
        let result = Solution::maximum_population(logs);
        let expected = 1993;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let logs = vec![vec![1950, 1961], vec![1960, 1971], vec![1970, 1981]];
        let result = Solution::maximum_population(logs);
        let expected = 1960;
        assert_eq!(result, expected);
    }
}
