#![allow(unused)]
pub struct Solution {}
// facebook interview
impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let chs = s.chars().collect::<Vec<char>>();
        let mut ret = vec!['a'; s.len()];
        for i in 0..indices.len() {
            ret[indices[i] as usize] = chs[i];
        }

        ret.into_iter().collect::<String>()
    }

    // cycle sort
    pub fn restore_string_cycle_sort(s: String, mut indices: Vec<i32>) -> String {
        let mut chs = s.chars().collect::<Vec<char>>();
        for i in 0..chs.len() {
            while indices[i] as usize != i {
                chs.swap(i, indices[i] as usize);
                let j = indices[i] as usize;
                indices.swap(i, j);
            }
        }

        chs.into_iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1528() {
        assert_eq!(
            Solution::restore_string_cycle_sort(
                "codeleet".to_string(),
                vec![4, 5, 6, 7, 0, 2, 1, 3]
            ),
            "leetcode".to_string()
        );
    }
}
