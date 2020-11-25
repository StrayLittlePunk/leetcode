#![allow(unused)]
pub struct Solution {}

impl Solution {
   pub fn remove_duplicate_letters(s: String) -> String {
        let mut counts: [usize; 26] = [0; 26];
        for &b in s.as_bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        let mut stack: Vec<char> = Vec::with_capacity(26);
        let mut exists: [bool; 26] = [false; 26];
        for &b in s.as_bytes() {
            let i = (b - b'a') as usize;
            counts[i] -= 1;
            if exists[i] {
                continue;
            }
            while let Some(&last) = stack.last() {
                let j = (last as u8 - b'a') as usize;
                if b < last as u8 && counts[j] > 0 {
                    exists[j] = false;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(b as char);
            exists[i] = true;
        }
        stack.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_293() {
        assert_eq!(
            Solution::remove_duplicate_letters("bcabc".to_string(),),
            "abc".to_string()
        );
        assert_eq!(
            Solution::remove_duplicate_letters("cbacdcbc".to_string(),),
            "acdb".to_string()
        );
    }
}
