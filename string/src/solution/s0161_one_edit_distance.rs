#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(n) O(n)
    pub fn is_one_edit_distance(s: String, t: String) -> bool {
        let (ns, nt) = (s.len(), t.len());

        if ns > nt {
            return Self::is_one_edit_distance(t, s);
        }

        if nt - ns > 1 {
            return false;
        }

        let s_str = s.as_str();
        let t_str = t.as_str();
        for i in 0..ns {
            if &s_str[i..i + 1] != &t_str[i..i + 1] {
                if ns == nt {
                    return &s_str[i + 1..] == &t_str[i + 1..];
                } else {
                    return &s_str[i..] == &t_str[i + 1..];
                }
            }
        }

        return ns + 1 == nt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_161() {
        assert_eq!(
            Solution::is_one_edit_distance("ab".to_string(), "acb".to_string()),
            true
        );
        assert_eq!(
            Solution::is_one_edit_distance("".to_string(), "".to_string()),
            false
        );
        assert_eq!(
            Solution::is_one_edit_distance("a".to_string(), "".to_string()),
            true
        );
        assert_eq!(
            Solution::is_one_edit_distance("".to_string(), "A".to_string()),
            true
        );
    }
}
