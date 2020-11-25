#![allow(unused)]
pub struct Solution {}
// apple interview

impl Solution {
    pub fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
        use std::collections::HashMap;
        let strsa = a.split(' ').collect::<Vec<&str>>();
        let strsb = b.split(' ').collect::<Vec<&str>>();
        let mut ans = vec![];
        let mut map = HashMap::new();

        for i in 0..strsa.len() {
            *map.entry(strsa[i]).or_insert(0) += 1;
        }
        for i in 0..strsb.len() {
            *map.entry(strsb[i]).or_insert(0) += 1;
        }

        for (&k, &v) in map.iter() {
            if v == 1 {
                ans.push(k.to_string());
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1629() {}
}
