#![allow(unused)]
pub struct Solution {}
use std::cmp::min;
impl Solution {
  // O(n) O(n)
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut rows = vec!["".to_string(); min(num_rows as usize, s.len())];
        let (mut cur, mut go_down) = (0, false);

        for c in s.chars() {
            rows[cur].push(c);
            if cur == 0 || cur == num_rows as usize - 1 {
                go_down = !go_down;
            }

            if go_down {
                cur += 1;
            } else {
                cur -= 1;
            }
        }

        rows.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI".to_string()
        );
        assert_eq!(Solution::convert("A".to_string(), 1), "A".to_string());
    }
}
