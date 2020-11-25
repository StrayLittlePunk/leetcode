#![allow(unused)]
pub struct Solution {}

use std::cmp::max;
impl Solution {
    // O(N) O(256)
    pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {

        let chs = s.chars().collect::<Vec<char>>();
        let (mut map, mut ans, mut slow, mut fast, mut counter) = ([0; 256], 0, 0, 0, 0);

        while fast < chs.len() {
            if map[chs[fast] as usize] == 0 {
                counter += 1;
            }
            map[chs[fast] as usize] += 1;
            fast += 1;

            while counter > 2 {
                if map[chs[slow] as usize] == 1 {
                    counter -= 1;
                }
                map[chs[slow] as usize] -= 1;
                slow += 1;
            }

            ans = max(ans, fast - slow);
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_159() {
        assert_eq!(
            Solution::length_of_longest_substring_two_distinct("eceba".to_owned()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring_two_distinct("ccaabbb".to_owned()),
            5
        );
    }
}
