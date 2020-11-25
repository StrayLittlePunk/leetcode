#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut stack = 0;
        let mut count = 0;
        for c in s.chars() {
            match c {
                'R' => stack += 1,
                'L' => stack -= 1,
                _ => panic!(),
            }
            if stack == 0 {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1221() {}
}
