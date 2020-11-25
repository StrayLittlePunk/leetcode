#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let chs = s.trim().chars().collect::<Vec<char>>();

        let (mut point_seen, mut e_seen, mut num_seen, mut num_after_e) =
            (false, false, false, true);

        for i in 0..chs.len() {
            if chs[i] >= '0' && chs[i] <= '9' {
                num_seen = true;
                num_after_e = true;
            } else if chs[i] == '.' {
                if e_seen || point_seen {
                    return false;
                }

                point_seen = true;
            } else if chs[i] == 'e' {
                if e_seen || !num_seen {
                    return false;
                }

                num_after_e = false;
                e_seen = true;
            } else if chs[i] == '-' || chs[i] == '+' {
                if i != 0 && chs[i - 1] != 'e' {
                    return false;
                }
            } else {
                return false;
            }
        }

        num_seen && num_after_e
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_65() {
        assert_eq!(Solution::is_number("0".to_string()), true);
        assert_eq!(Solution::is_number("a".to_string()), false);
    }
}
