#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {

        
        match s.trim().rsplit(' ').next() {
            Some(sr) => sr.len() as i32,
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_58() {
        assert_eq!(Solution::length_of_last_word("hello world".to_string()), 5);
        assert_eq!(Solution::length_of_last_word(" world".to_string()), 5);
        assert_eq!(Solution::length_of_last_word("world ".to_string()), 5);
    }
}
