#![allow(unused)]
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
  // O(N) O(n)
    pub fn min_window(s: String, t: String) -> String {
        if s.len() == 0 || t.len() == 0 {
            return "".to_owned();
        }

        let mut map = HashMap::new();
        for ch in t.chars() {
            let count = map.entry(ch).or_insert(0);
            *count += 1;
        }

        let required = map.len();

        let (mut left, mut right, mut formed) = (0, 0, 0);

        let mut window_cnts = HashMap::new();
        let mut ans: [i32; 3] = [-1, 0, 0];
        let chs = s.chars().collect::<Vec<char>>();
        while right < s.len() {
            let ch = chs[right];
            let count = window_cnts.entry(ch).or_insert(0);
            *count += 1;

            if map.contains_key(&ch) && *window_cnts.get(&ch).unwrap() == *map.get(&ch).unwrap() {
                formed += 1;
            }

            while left <= right && formed == required {
                let ch = chs[left];
                if ans[0] == -1 || ((right - left + 1) as i32) < ans[0] {
                    ans[0] = (right - left + 1) as i32;
                    ans[1] = left as i32;
                    ans[2] = right as i32;
                }

                if let Some(cnt) = window_cnts.get_mut(&ch) {
                    *cnt -= 1;
                }

                if map.contains_key(&ch) && *window_cnts.get(&ch).unwrap() < *map.get(&ch).unwrap()
                {
                    formed -= 1;
                }

                left += 1;
            }

            right += 1;
        }

        if ans[0] == -1 {
            "".to_owned()
        } else {
            let ret = s.as_str();
            ret[ans[1] as usize..ans[2] as usize + 1].to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_76() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_owned(), "ABC".to_owned()),
            "BANC".to_owned()
        );
    }
}
