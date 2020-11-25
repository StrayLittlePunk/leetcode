#![allow(unused)]
pub struct Solution {}
use std::cmp::max;
use std::collections::HashMap;

// thanks for Pochmann idea
// https://leetcode.com/problems/longest-absolute-file-path/discuss/86619/Simple-Python-solution
impl Solution {
    // Time O(N), Space O(1) N is len of path string
    pub fn length_longest_path(input: String) -> i32 {
        let mut dict: HashMap<i32, i32> = HashMap::new();
        dict.insert(0, 0);
        let mut maxlen = 0;
        for line in input.split("\n") {
            let name = &line.trim_matches('\t');
            let depth: i32 = (line.len() - name.len()) as i32;
            if name.contains(".") {
                maxlen = max(maxlen, *dict.get(&depth).unwrap() + name.len() as i32);
            } else {
                let tmp = *dict.get(&depth).unwrap() + name.len() as i32 + 1;
                dict.insert(depth + 1, tmp);
            }
        }

        maxlen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_388() {
        assert_eq!(
            Solution::length_longest_path("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext".to_string()),
            20
        );
        assert_eq!(
            Solution::length_longest_path(
              "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext".to_string()),
            32
        );
        assert_eq!(Solution::length_longest_path("a".to_string()), 0);
    }
}
