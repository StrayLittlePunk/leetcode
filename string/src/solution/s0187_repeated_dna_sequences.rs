#![allow(unused)]
pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    // O((n-l) * l)  O((n-l) * l)  l = 10
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() <= 10 {
            return vec![];
        }
        let mut all: HashSet<&str> = HashSet::new();
        let mut rep: HashSet<&str> = HashSet::new();

        let mut l = 0;
        let mut r = 10;

        while r <= s.len() {
            if all.contains(&s[l..r]) {
                rep.insert(&s[l..r]);
            } else {
                all.insert(&s[l..r]);
            }
            l += 1;
            r += 1;
        }

        let ret: Vec<String> = rep.iter().map(|i| i.to_string()).collect();
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_187() {
        assert_eq!(
            Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_owned()),
            vec!["CCCCCAAAAA".to_owned(), "AAAAACCCCC".to_owned(),]
        );
    }
}
