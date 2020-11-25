#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut left = 0;
        let mut ret = vec![];
        let max_width = max_width as usize;

        while left < words.len() {
            let right = Self::find_right(left, &words, max_width);
            ret.push(Self::justify(left, right, &words, max_width));
            left = right + 1;
        }

        ret
    }

    fn find_right(left: usize, words: &Vec<String>, max_width: usize) -> usize {
        let mut right = left;
        let mut sum = words[right].len();

        right += 1;

        while right < words.len() && (sum + 1 + words[right].len()) <= max_width {
            sum += 1 + words[right].len();
            right += 1;
        }

        right - 1
    }

    fn justify(left: usize, right: usize, words: &Vec<String>, max_width: usize) -> String {
        if right == left {
            return Self::pad_ret(words[left].clone(), max_width);
        }
        let is_last_line = right == words.len() - 1;
        let num_spaces = right - left;
        let total_space = max_width - Self::words_len(left, right, words);

        let mut remainder = if is_last_line {
            0
        } else {
            total_space % num_spaces
        };

        let mut ret = "".to_owned();
        for i in left..=right {
            ret.push_str(words[i].as_str());
            if is_last_line {
                ret.push(' ');
            } else {
                for _ in 0..(total_space / num_spaces) as usize {
                    ret.push(' ');
                }
            }
            if remainder > 0 {
                ret.push(' ');
            }
            if remainder == 0 {
                continue;
            }
            remainder -= 1;
        }

        Self::pad_ret(ret.trim().to_owned(), max_width)
    }

    fn words_len(left: usize, right: usize, words: &Vec<String>) -> usize {
        let mut words_len = 0;
        for i in left..=right {
            words_len += words[i].len();
        }

        words_len
    }

    fn pad_ret(mut ret: String, max_width: usize) -> String {
        for _ in 0..max_width as usize - ret.len() {
            ret.push(' ');
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_68() {
        assert_eq!(
            Solution::full_justify(
                vec![
                    "This".to_owned(),
                    "is".to_owned(),
                    "an".to_owned(),
                    "example".to_owned(),
                    "of".to_owned(),
                    "text".to_owned(),
                    "justification.".to_owned(),
                ],
                16
            ),
            vec![
                "This    is    an".to_owned(),
                "example  of text".to_owned(),
                "justification.  ".to_owned(),
            ]
        );
        assert_eq!(
            Solution::full_justify(
                vec![
                    "What".to_owned(),
                    "must".to_owned(),
                    "be".to_owned(),
                    "acknowledgment".to_owned(),
                    "shall".to_owned(),
                    "be".to_owned(),
                ],
                16
            ),
            vec![
                "What   must   be".to_owned(),
                "acknowledgment  ".to_owned(),
                "shall be        ".to_owned(),
            ]
        );

        assert_eq!(
            Solution::full_justify(
                vec![
                    "Science".to_owned(),
                    "is".to_owned(),
                    "what".to_owned(),
                    "we".to_owned(),
                    "understand".to_owned(),
                    "well".to_owned(),
                    "enough".to_owned(),
                    "to".to_owned(),
                    "explain".to_owned(),
                    "to".to_owned(),
                    "a".to_owned(),
                    "computer.".to_owned(),
                    "Art".to_owned(),
                    "is".to_owned(),
                    "everything".to_owned(),
                    "else".to_owned(),
                    "we".to_owned(),
                    "do".to_owned(),
                ],
                20
            ),
            vec![
                "Science  is  what we".to_owned(),
                "understand      well".to_owned(),
                "enough to explain to".to_owned(),
                "a  computer.  Art is".to_owned(),
                "everything  else  we".to_owned(),
                "do                  ".to_owned(),
            ]
        );
    }
}
