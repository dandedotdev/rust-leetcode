// <Array, Counting, Prefix Sum>
// Time: O(n + m), n is the number of logs, m is the duration
// Space: O(1)

// Idiomatic

pub struct Solution;

impl Solution {
    const DURATION: usize = 2050 - 1950 + 1;
    const YEAR_START: usize = 1950;

    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        logs.iter()
            .flat_map(|log| {
                let (start, end) = (log[0] as usize, log[1] as usize);
                start..end
            })
            .fold(vec![0; Self::DURATION], |mut timeline, year| {
                timeline[year - Self::YEAR_START] += 1;
                timeline
            })
            .into_iter()
            .enumerate()
            .max_by_key(|&(year, population)| (population, -(year as i32))) // if population is the same, choose the year with the smaller index
            .map(|(year, _)| (year + Self::YEAR_START) as i32)
            .unwrap_or(0)
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
