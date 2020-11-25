#![allow(unused)]
pub struct Solution {}

use std::cmp::max;
impl Solution {
    // O(N) O(1)
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let chs = s.chars().collect::<Vec<char>>();
        let (mut map, max_unique, mut ret) = ([0; 26], Self::get_max_uniqueletter(&chs), 0);

        for cur in 1..=max_unique {
            // reset map;
            for i in 0..map.len() {
                map[i] = 0;
            }

            let (mut start, mut end, mut idx, mut unique, mut count) = (0, 0, 0, 0, 0);
            while end < chs.len() {
                // expand sliding window
                if unique <= cur {
                    idx = (chs[end] as u32 - 'a' as u32) as usize;
                    if map[idx] == 0 {
                        unique += 1;
                    }
                    map[idx] += 1;
                    if map[idx] == k {
                        count += 1;
                    }
                    end += 1;
                }
                // shrink sliding window
                else {
                    idx = (chs[start] as u32 - 'a' as u32) as usize;
                    if map[idx] == k {
                        count -= 1;
                    }
                    map[idx] -= 1;
                    if map[idx] == 0 {
                        unique -= 1;
                    }
                    start += 1;
                }

                if unique == cur && unique == count {
                    ret = max(end - start, ret);
                }
            }
        }

        ret as i32
    }

    fn get_max_uniqueletter(chs: &Vec<char>) -> usize {
        let (mut map, mut max_unique) = ([false; 26], 0);
        for i in 0..chs.len() {
            if !map[(chs[i] as u32 - 'a' as u32) as usize] {
                max_unique += 1;
                map[(chs[i] as u32 - 'a' as u32) as usize] = true;
            }
        }

        max_unique
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_395() {
        assert_eq!(Solution::longest_substring("ababbc".to_owned(), 2), 5);
        assert_eq!(Solution::longest_substring("aaabb".to_owned(), 3), 3);
    }
}
