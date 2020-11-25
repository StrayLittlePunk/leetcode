#![allow(unused)]
pub struct Solution {}

use std::cmp::max;
use std::collections::HashMap;
impl Solution {

    // O(n) O(256)
    pub fn length_of_longest_substring_k_distinct_ascii(s: String, k: i32) -> i32 {
        let chs = s.chars().collect::<Vec<char>>();
        let (mut map, mut ans, mut i, mut j, mut counter, k) = ([0; 256], 0, 0, 0, 0, k as usize);

        while j < chs.len() {
            if map[chs[j] as usize] == 0 {
                counter += 1;
            }
            map[chs[j] as usize] += 1;
            j += 1;

            while counter > k {
                if map[chs[i] as usize] == 1 {
                    counter -= 1;
                }

                map[chs[i] as usize] -= 1;
                i += 1;
            }

            ans = max(ans, j - i);
        }

        ans as i32
    }
    // O(n) O(k)
    pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
        let chs = s.chars().collect::<Vec<char>>();
        let (mut map, mut ans, mut i, mut j, k) = (HashMap::new(), 0, 0, 0, k as usize);

        while j < chs.len() {
            let count = map.entry(chs[j]).or_insert(0);
            *count += 1;
            j += 1;

            if map.len() > k {
                let val = *map.get(&chs[i]).unwrap();
                if val - 1 == 0 {
                    map.remove(&chs[i]);
                } else {
                    map.insert(chs[i], val - 1);
                }
                i += 1;
            }

            ans = max(ans, j - i);
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_340() {
        assert_eq!(
            Solution::length_of_longest_substring_k_distinct("aa".to_owned(), 1),
            2
        );
        assert_eq!(
            Solution::length_of_longest_substring_k_distinct("eceba".to_owned(), 2),
            3
        );
    }
}
