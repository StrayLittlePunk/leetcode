#![allow(unused)]
pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn generate_palindromes(s: String) -> Vec<String> {
        let mut map = [0; 256];
        let mut st = vec!['0'; s.len() / 2];
        let mut set = HashSet::new();
        if !Self::can_permute_palindrome(s.as_str(), &mut map) {
            return vec![];
        }

        let mut ch = std::char::from_u32(0).unwrap();
        let mut k = 0;
        for i in 0..map.len() {
            if map[i] % 2 == 1 {
                ch = std::char::from_u32(i as u32).unwrap();
            }
            for j in 0..(map[i] / 2) as usize {
                st[k] = std::char::from_u32(i as u32).unwrap();
                k += 1;
            }
        }

        Self::permute(&mut st, 0, ch, &mut set);

        return set.into_iter().collect::<Vec<String>>();
    }

    fn permute(st: &mut Vec<char>, left: usize, ch: char, set: &mut HashSet<String>) {
        if left == st.len() {
            let mut ret = st.iter().collect::<String>();
            if ch as u32 != 0 {
              ret.push(ch);
            }
            st.iter().rev().for_each(|&x| ret.push(x));
            set.insert(ret);
        } else {
            for i in left..st.len() {
                if st[left] != st[i] || left == i {
                    Self::swap(st, left, i);
                    Self::permute(st, left + 1, ch, set);
                    Self::swap(st, left, i);
                }
            }
        }
    }

    fn swap(st: &mut Vec<char>, left: usize, right: usize) {
        let temp = st[left];
        st[left] = st[right];
        st[right] = temp;
    }

    fn can_permute_palindrome(s: &str, map: &mut [i32; 256]) -> bool {
        let mut count = 0;
        for ch in s.chars() {
            map[(ch as u32) as usize] += 1;
            if map[(ch as u32) as usize] % 2 == 0 {
                count -= 1;
            } else {
                count += 1;
            }
        }
        count <= 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_267() {
        assert_eq!(Solution::generate_palindromes("abc".to_owned()), Vec::<String>::new());
        assert_eq!(
            Solution::generate_palindromes("aabb".to_owned()),
            vec!["abba".to_owned(), "baab".to_owned(),]
        );
    }
}
