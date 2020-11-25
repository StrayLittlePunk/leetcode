#![allow(unused)]
pub struct Solution {}
// amazon interview
impl Solution {
    pub fn num_k_len_substr_no_repeats_hashmap(s: String, k: i32) -> i32 {
        use std::collections::HashMap;
        if s.len() < k as usize {
            return 0;
        }
        let (mut ans, mut map, chs, k) = (0, HashMap::new(), s.chars().collect::<Vec<char>>(), k as usize);
        
        // begin k element whether no repeated chars
        for i in 0..k {
            *map.entry(chs[i]).or_insert(0) += 1;
        }
        // if yes , ans += 1;
        if map.len() == k {
            ans += 1;
        }
        // check others chars
        for i in k..chs.len() {
            *map.entry(chs[i - k]).or_insert(0) -= 1;
            if *map.get(&chs[i - k]).unwrap() == 0 {
                map.remove(&chs[i-k]);
            }
            *map.entry(chs[i]).or_insert(0) += 1;
            if map.len() == k {
                ans += 1;
            }
        }
        
        ans
    }
    
    pub fn num_k_len_substr_no_repeats(s: String, k: i32) -> i32 {
        use std::collections::HashSet;
        let (mut ret, mut i, mut cur, chs, k) = (0, 0, HashSet::new(), s.chars().collect::<Vec<char>>(), k as usize);
        for j in 0..s.len() {
            while cur.contains(&chs[j]) {
                cur.remove(&chs[i]);
                i += 1;
            }
            cur.insert(chs[j]);
            ret += if j - i + 1 >= k {1} else {0};
        }
        ret
    }
}

 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1100() {}
}
