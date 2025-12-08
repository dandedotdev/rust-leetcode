// <Enumeration, Math>
// Time: O(n^2)
// Space: O(n^2)

pub struct Solution;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        if n <= 4 {
            return 0;
        }
        let (len, max_sum) = (n as usize, (n * n) as usize);
        let mut squares = Vec::with_capacity(len);
        let mut squares_check = vec![false; max_sum + 1];
        (1..=len).map(|x| x * x).for_each(|x| {
            squares_check[x] = true;
            squares.push(x);
        });
        let mut count = 0;
        for (i, &a) in squares[..len - 2].iter().enumerate() {
            for &b in &squares[i + 1..len - 1] {
                match a + b {
                    sum if sum <= max_sum => count += squares_check[sum] as i32 * 2,
                    _ => break,
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 5;
        let result = Solution::count_triples(n);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let n = 10;
        let result = Solution::count_triples(n);
        let expected = 4;
        assert_eq!(result, expected);
    }
}
