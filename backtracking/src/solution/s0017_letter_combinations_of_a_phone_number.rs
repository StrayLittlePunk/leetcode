#![allow(unused)]
pub struct Solution {}

impl Solution {
    fn backtrack(
        pattern: &Vec<usize>,
        table: &Vec<Vec<char>>,
        ans: &mut Vec<String>,
        cur: &mut String,
        idx: usize,
    ) {
        if idx == pattern.len() {
            ans.push(cur.clone());
            return;
        }

        for &ch in table[pattern[idx]].iter() {
            cur.push(ch);
            Self::backtrack(pattern, table, ans, cur, idx + 1);
            cur.pop();
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut ans = vec![];
        if digits.is_empty() {
            return ans;
        }

        let mut letters_tbl = vec![vec![], vec![]];
        letters_tbl.push(vec!['a', 'b', 'c']);
        letters_tbl.push(vec!['d', 'e', 'f']);
        letters_tbl.push(vec!['g', 'h', 'i']);
        letters_tbl.push(vec!['j', 'k', 'l']);
        letters_tbl.push(vec!['m', 'n', 'o']);
        letters_tbl.push(vec!['p', 'q', 'r', 's']);
        letters_tbl.push(vec!['t', 'u', 'v']);
        letters_tbl.push(vec!['w', 'x', 'y', 'z']);

        let pattern = digits
            .chars()
            .map(|c| (c as u32 - '0' as u32) as usize)
            .collect::<Vec<usize>>();
        let mut cur = String::from("");

        Self::backtrack(&pattern, &letters_tbl, &mut ans, &mut cur, 0);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_17() {
        assert_eq!(
            Solution::letter_combinations("23".to_owned()),
            vec![
                "ad".to_owned(),
                "ae".to_owned(),
                "af".to_owned(),
                "bd".to_owned(),
                "be".to_owned(),
                "bf".to_owned(),
                "cd".to_owned(),
                "ce".to_owned(),
                "cf".to_owned(),
            ]
        );
        assert_eq!(Solution::letter_combinations("".to_owned()), Vec::<String>::new());
        assert_eq!(
            Solution::letter_combinations("2".to_owned()),
            vec!["a".to_owned(), "b".to_owned(), "c".to_owned(),]
        );
    }
}
