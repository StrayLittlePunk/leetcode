#![allow(unused)]
pub struct Solution {}

impl Solution {

    pub fn shortest_palindrome(s: String) -> String {
        if s.len() == 0 {
            return s;
        }

        let ss: Vec<char> = s.chars().chain(s.chars().rev()).collect();
        let b = Self::kmp(&ss);
        let mut max_pre = b[ss.len()] as usize;
        while max_pre > s.len() {
            max_pre = b[max_pre] as usize;
        }
        let r = s[max_pre..].chars().rev().chain(s.chars()).collect();
        r
    }

    fn kmp(s: &Vec<char>) -> Vec<i32> {
        // assert s.len() >= 1
        let mut b = vec![0; s.len() + 1];
        b[0] = -1;
        b[1] = 0;
        for i in 1..s.len() {
            let mut j = i;
            loop {
                if j == 0 || s[b[j] as usize] == s[i] {
                    b[i + 1] = b[j] + 1;
                    break;
                } else {
                    j = b[j] as usize;
                }
            }
        }
        b
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_214() {
        assert_eq!(
            Solution::shortest_palindrome("aacecaaa".to_owned()),
            "aaacecaaa".to_owned()
        );
        assert_eq!(
            Solution::shortest_palindrome("abcd".to_owned()),
            "dcbabcd".to_owned()
        );
    }
}
