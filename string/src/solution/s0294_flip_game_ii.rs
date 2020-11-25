#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn can_win(s: String) -> bool {
        let mut chars = s.chars().collect::<Vec<char>>();
        Self::backtrack(&mut chars)
    }

    fn backtrack(chs: &mut Vec<char>) -> bool {
        for i in 1..chs.len() {
            if chs[i] == '+' && chs[i - 1] == '+' {
                chs[i] = '-';
                chs[i - 1] = '-';
                let ret = !Self::backtrack(chs);
                chs[i] = '+';
                chs[i - 1] = '+';

                if ret {
                    return true;
                }
            }
        }

        false
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_294() {
        assert_eq!(Solution::can_win("++++".to_string(),), true);
    }
}
