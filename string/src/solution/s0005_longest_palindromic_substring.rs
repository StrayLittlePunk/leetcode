#![allow(unused)]
pub struct Solution {}

use std::cmp::max;
impl Solution {
    // O(N^2) O(N^2)

    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let chs = s.chars().collect::<Vec<char>>();
        let mut dp = vec![vec![false; chs.len()]; chs.len()];
        let (mut end, mut begin) = (0, 0);

        for j in 0..chs.len() {
            // Optional
            dp[j][j] = true;

            for i in 0..=j {
                dp[i][j] = (chs[i] == chs[j] && (j - i < 2 || j > 0 && dp[i + 1][j - 1]));
                if dp[i][j] && (j - i + 1) > (end - begin) {
                    begin = i;
                    end = j + 1;
                }
            }
        }

        chs[begin..end].iter().collect::<String>()
    }

    // O(N^2) O(1)
    pub fn longest_palindrome_middle_out(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let chs = s.chars().collect::<Vec<char>>();
        let (mut end, mut begin) = (0, 0);
        for i in 0..chs.len() {
            let odd_len = Self::is_palindrome(&chs, i as i32, i as i32);
            let even_len = Self::is_palindrome(&chs, i as i32, i as i32 + 1);

            let len = max(odd_len, even_len);
            if len > end - begin {
                begin = i - (len - 1) / 2;
                end = i + len / 2;
            }
        }

        chs[begin..end+1].iter().collect::<String>()
    }

    fn is_palindrome(chs: &Vec<char>, mut left: i32, mut right: i32) -> usize {
        while left >= 0 && right < chs.len() as i32 && chs[left as usize] == chs[right as usize] {
            left -= 1;
            right += 1;
        }

        (right - left - 1) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        assert_eq!(
            Solution::longest_palindrome_middle_out("babad".to_owned()),
            "aba".to_owned()
        );
        assert_eq!(
            Solution::longest_palindrome_middle_out("cbbd".to_owned()),
            "bb".to_owned()
        );
        assert_eq!(Solution::longest_palindrome_middle_out("a".to_owned()), "a".to_owned());
        assert_eq!(
            Solution::longest_palindrome_middle_out("ac".to_owned()),
            "c".to_owned()
        );
        assert_eq!(
            Solution::longest_palindrome("babad".to_owned()),
            "bab".to_owned()
        );
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_owned()),
            "bb".to_owned()
        );
        assert_eq!(Solution::longest_palindrome("a".to_owned()), "a".to_owned());
        assert_eq!(
            Solution::longest_palindrome("ac".to_owned()),
            "a".to_owned()
        );
    }
}
