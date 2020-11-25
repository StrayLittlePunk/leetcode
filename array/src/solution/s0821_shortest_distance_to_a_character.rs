#![allow(unused)]
pub struct Solution {}

// apple interview
impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        use std::cmp::min;
        use std::i32::{MIN, MAX};
        let chs = s.chars().collect::<Vec<char>>();
        let mut ans = vec![0;chs.len()];
        let mut prev = MIN / 2;
        
        for i in 0..chs.len() {
            if chs[i] == c {
                prev = i as i32;
            }
            ans[i] = i as i32 - prev;
        }
        
        prev = MAX / 2;
        for i in (0..chs.len()).rev() {
            if chs[i] == c {
                prev = i as i32;
            }
            ans[i] = min(ans[i], prev - i as i32);
        }
        
        ans

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_821() {
    }
}
