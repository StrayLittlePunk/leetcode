#![allow(unused)]
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn word_pattern_match(pattern: String, s: String) -> bool {
        if pattern.len() == 0 && s.len() == 0 {
            return true;
        } else if pattern.len() == 0 || s.len() == 0 {
            return false;
        } else if pattern.len() == 1 {
            return true;
        }

        let pattern = pattern.chars().collect::<Vec<char>>();

        // Pre-allocate the shared data structures for the search
        let mut pat_map = HashMap::with_capacity(pattern.len());
        let mut str_map = HashMap::with_capacity(pattern.len());
        let mut solution = Vec::with_capacity(s.len());

        Self::search(
            &pattern,
            &s,
            0,
            0,
            &mut pat_map,
            &mut str_map,
            &mut solution,
        )
    }

    fn search<'a>(
        pattern: &[char],
        s: &'a str,
        pat_idx: usize,
        str_idx: usize,
        pat_map: &mut HashMap<char, &'a str>,
        str_map: &mut HashMap<&'a str, char>,
        solution: &mut Vec<&'a str>,
    ) -> bool {
        if str_idx == s.len() && solution.len() == pattern.len() {
            // We have a solution
            return true;
        } else if str_idx == s.len() || pat_idx == pattern.len() {
            // Reached the end of string or pattern without a solution
            return false;
        }

        let c = pattern[pat_idx];

        // Iterate over possible end positions for this mapping
        for end in str_idx..s.len() {
            let mapping = &s[str_idx..=end];
            let found;

            if let Some(old_mapping) = pat_map.get(&c) {
                // No way to find the existing mapping, so stop
                if mapping.len() > old_mapping.len() {
                    break;
                }

                // Current mapping is not equal to existing one; try next mapping
                if old_mapping != &mapping {
                    continue;
                }

                found = true;
            } else if let Some(res) = str_map.get(mapping) {
                // Mapping is already used, try next one
                if res != &c {
                    continue;
                }

                found = true;
            } else {
                found = false;
                pat_map.insert(c, mapping);
                str_map.insert(mapping, c);
            }

            solution.push(mapping);

            if Self::search(pattern, s, pat_idx + 1, end + 1, pat_map, str_map, solution) {
                return true;
            }

            solution.pop();

            // Revert the mappings
            if !found {
                pat_map.remove(&c);
                str_map.remove(mapping);
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_291() {
        assert_eq!(
            Solution::word_pattern_match("abab".to_owned(), "redblueredblue".to_owned()),
            true
        );
        assert_eq!(
            Solution::word_pattern_match("aaaa".to_owned(), "asdasdasdasd".to_owned()),
            true
        );
        assert_eq!(
            Solution::word_pattern_match("abab".to_owned(), "asdasdasdasd".to_owned()),
            true
        );
        assert_eq!(
            Solution::word_pattern_match("aabb".to_owned(), "xyzabcxzyabc".to_owned()),
            false
        );
    }
}
