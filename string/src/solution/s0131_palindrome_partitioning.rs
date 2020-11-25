#![allow(unused)]
pub struct Solution {}

impl Solution {
    // O(N* 2^n) O(N^2)
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut dp_vec: Vec<Vec<bool>> = vec![vec![false; s.len()]; s.len()];
        for i in 0..s.len() {
            for j in 0..=i {
                if s.chars().nth(j).unwrap() == s.chars().nth(i).unwrap()
                    && (i - j <= 2 || dp_vec[j + 1][i - 1])
                {
                    dp_vec[j][i] = true;
                }
            }
        }
        let mut ret: Vec<Vec<String>> = vec![];
        let mut buffer: Vec<String> = vec![];
        Solution::helper(&mut ret, &mut buffer, &s, &dp_vec, 0);
        ret
    }

    fn helper(
        ret: &mut Vec<Vec<String>>,
        buffer: &mut Vec<String>,
        s: &String,
        dp_vec: &Vec<Vec<bool>>,
        footer: usize,
    ) {
        if footer == s.len() {
            ret.push(buffer.to_vec().clone());
            return;
        }
        for i in footer..s.len() {
            if (dp_vec[footer][i]) {
                buffer.push(s[footer..i + 1].to_string());
                Solution::helper(ret, buffer, s, dp_vec, i + 1);
                buffer.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_125() {
        assert_eq!(
            Solution::partition("aab".to_owned()),
            vec![
                vec!["a".to_owned(), "a".to_owned(), "b".to_owned(),],
                vec!["aa".to_owned(), "b".to_owned(),],
            ]
        );
    }
}
