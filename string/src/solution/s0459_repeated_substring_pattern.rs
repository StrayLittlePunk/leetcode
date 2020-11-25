#![allow(unused)]
pub struct Solution {}
// google interview

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let n = s.len();
        let mut dp = vec![0;n];
        // Construct partial match table (lookup table).
        // It stores the length of the proper prefix that is also a proper suffix.
        // ex. ababa --> [0, 0, 1, 2, 1]
        // ab --> the length of common prefix / suffix = 0
        // aba --> the length of common prefix / suffix = 1
        // abab --> the length of common prefix / suffix = 2
        // ababa --> the length of common prefix / suffix = 1
        let chs = s.chars().collect::<Vec<char>>();
        for i in 1..n {
            let mut j = dp[i -1];
            while j > 0 && chs[i] != chs[j] {
                j = dp[j - 1];
            }
            if chs[i] == chs[j] {
                j += 1;
            }
            dp[i] = j;
        }
        let l = dp[n - 1];
        // check if it's repeated pattern string
        return l != 0 && n % (n - l) == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1002() {}
}
