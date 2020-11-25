#![allow(unused)]
pub struct Solution {}
const PAIRS: [(char, char); 5] = [('0', '0'), ('1', '1'), ('6', '9'), ('8', '8'), ('9', '6')];
impl Solution {
    // O(5^(n/2)) O(n)
    pub fn strobogrammatic_in_range(low: String, high: String) -> i32 {
        let mut count = [0];
        for len in low.len()..=high.len() {
            let mut chs = vec!['0'; len];
            Self::dfs(&low, &high, &mut chs, 0, len as i32 - 1, &mut count);
        }

        count[0]
    }

    fn dfs(
        low: &str,
        high: &str,
        chs: &mut Vec<char>,
        left: i32,
        right: i32,
        count: &mut [i32],
    ) {
        if left > right {
            let s = chs.iter().collect::<String>();
            if (s.len() == low.len() && s.as_str() < low)
                || (s.len() == high.len() && s.as_str() > high)
            {
                return;
            }

            count[0] += 1;
            return;
        }

        for pair in PAIRS.iter() {
            chs[left as usize] = pair.0;
            chs[right as usize] = pair.1;
            if chs.len() != 1 && chs[0] == '0' {
                continue;
            }
            if left == right && pair.0 != pair.1 {
                continue;
            }
            Self::dfs(low, high, chs, left + 1, right - 1, count);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_246() {
        assert_eq!(
            Solution::strobogrammatic_in_range("0".to_owned(), "0".to_owned()),
            1
        );
        assert_eq!(
            Solution::strobogrammatic_in_range("50".to_owned(), "100".to_owned()),
            3
        );
    }
}
