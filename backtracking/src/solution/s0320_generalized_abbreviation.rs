#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn generate_abbreviations(word: String) -> Vec<String> {
        let mut ans = vec![];
        let chs = word.chars().collect::<Vec<char>>();
        for x in 0..(1 << word.len()) {
            ans.push(Self::abbr(&chs, x));
        }

        ans
    }

    fn abbr(word: &Vec<char>, mut x: usize) -> String {
        let mut sb = String::from("");
        let mut k = 0;

        for i in 0..word.len() {
            if (x & 1) == 0 {
                // bit is zero
                if k != 0 {
                    sb += &k.to_string();
                    k = 0;
                }
                sb.push(word[i]);
            } else {
                k += 1;
            }

            x >>= 1;
        }

        if k != 0 {
            sb += &k.to_string();
        }

        sb
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_320() {
        assert_eq!(
            Solution::generate_abbreviations("word".to_owned()),
            vec![
                "word".to_owned(),
                "1ord".to_owned(),
                "w1rd".to_owned(),
                "2rd".to_owned(),
                "wo1d".to_owned(),
                "1o1d".to_owned(),
                "w2d".to_owned(),
                "3d".to_owned(),
                "wor1".to_owned(),
                "1or1".to_owned(),
                "w1r1".to_owned(),
                "2r1".to_owned(),
                "wo2".to_owned(),
                "1o2".to_owned(),
                "w3".to_owned(),
                "4".to_owned(),
            ]
        );
    }
}
