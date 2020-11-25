#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn reverse_words(s: &mut Vec<char>) {
        s.reverse();
        let (mut start, mut end) = (0, 0);
        while start < s.len() {
            //find word end index;
            end = start;
            while end < s.len() && s[end] != ' ' {
                end += 1;
            }


            Self::reverse_word(s, start, end - 1);
            start = end + 1;
        }
    }

    fn reverse_word(s: &mut Vec<char>, mut start: usize, mut end: usize) {
        while start < end {
            let tmp = s[start];
            s[start] = s[end];
            s[end] = tmp;

            start += 1;
            end -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_151() {
        let mut s = vec![
            't', 'h', 'e', ' ', 's', 'k', 'y', ' ', 'i', 's', ' ', 'b', 'l', 'u', 'e',
        ];
        Solution::reverse_words(&mut s);
        assert_eq!(
            s,
            vec!['b', 'l', 'u', 'e', ' ', 'i', 's', ' ', 's', 'k', 'y', ' ', 't', 'h', 'e']
        );
    }
}
