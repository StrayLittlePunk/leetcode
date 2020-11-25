#![allow(unused)]
pub struct Solution {}

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        if s.len() == 0 {
            return "".to_string();
        }
        let mut chs: Vec<char> = s.chars().collect();

        let (mut lptr, mut rptr) = (0, chs.len() - 1);
        while lptr < rptr {
            if !(chs[lptr].is_ascii_alphabetic() && Self::is_vowels(chs[lptr].to_ascii_lowercase()))
            {
                lptr += 1;
                continue;
            }
            if !(chs[rptr].is_ascii_alphabetic() && Self::is_vowels(chs[rptr].to_ascii_lowercase()))
            {
                rptr -= 1;
                continue;
            }

            let tmp = chs[lptr];
            chs[lptr] = chs[rptr];
            chs[rptr] = tmp;

            lptr += 1;
            rptr -= 1;
        }

        chs.into_iter().collect()
    }

    fn is_vowels(ch: char) -> bool {
        for vowel in VOWELS.iter() {
            if *vowel == ch {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_345() {
        assert_eq!(
            Solution::reverse_vowels("hello".to_string(),),
            "holle".to_string(),
        );
        assert_eq!(
            Solution::reverse_vowels("leetcode".to_string(),),
            "leotcede".to_string(),
        );
    }
}
