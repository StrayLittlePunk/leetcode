#![allow(unused)]
pub struct Solution {}

use std::cmp::max;
use std::collections::{HashMap, HashSet};
impl Solution {
    // O(2n) O(min(m, n))
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chs = s.chars().collect::<Vec<char>>();
        let mut set = HashSet::new();
        let (mut ans, mut slow, mut fast) = (0, 0, 0);
        while slow < chs.len() && fast < chs.len() {
            // try to extend the range [slow, fast]
            if !set.contains(&chs[fast]) {
                set.insert(chs[fast]);
                fast += 1;
                ans = max(ans, fast - slow);
            } else {
                set.remove(&chs[slow]);
                slow += 1;
            }
        }

        ans as i32
    }
    // O(n) O(min(m, n))
    pub fn length_of_longest_substring_faster(s: String) -> i32 {
        let chs = s.chars().collect::<Vec<char>>();
        // current index of character
        let (mut map, mut ans, mut i, mut j) = (HashMap::new(), 0, 0, 0);
        // try to extend the range [i, j]
        while j < chs.len() {
            if map.contains_key(&chs[j]) {
                i = max(*map.get(&chs[j]).unwrap(), i);
            }

            j += 1;
            ans = max(ans, j - i);
            map.insert(chs[j - 1], j);
        }

        ans as i32
    }
    // int[26] for Letters 'a' - 'z' or 'A' - 'Z 
    // int[128] for ASCII
    // int[256] for Extended ASCII
    pub fn length_of_longest_substring_ascii(s: String) -> i32 {
        let chs = s.chars().collect::<Vec<char>>();
        let (mut map, mut ans, mut i, mut j) = ([0;256], 0, 0, 0);

        while j < chs.len() {
          i = max(map[chs[j] as usize], i);
          j += 1;

          ans = max(ans, j - i);
          map[chs[j - 1] as usize] = j 
        }

        ans as i32

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::length_of_longest_substring_faster("abcabcbb".to_owned()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring_faster("bbbbb".to_owned()), 1);
        assert_eq!(
            Solution::length_of_longest_substring_faster("pwwkew".to_owned()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring_faster("".to_owned()), 0);
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_owned()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_owned()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_owned()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("".to_owned()), 0);
    }
}
