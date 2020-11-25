#![allow(unused)]
pub struct Solution {}

// facebook interview

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let chs = s.chars().collect::<Vec<char>>();
        for i in 0..chs.len()/2 {
            if chs[i] != chs[chs.len() - i - 1] {
                let right = s.len() - i -1;
                // delete (i+1) left char or delete (right - 1) right char to check substr is palindrome
                return Self::is_palindrome_range(&chs, i + 1, right) || Self::is_palindrome_range(&chs, i, right - 1);
            }
        }
        // don't delete is palindrome
        return true;
    }

    // check given range chars is palindrome
    fn is_palindrome_range(chs: &Vec<char>, left: usize, right: usize) -> bool {
        for k in left..=(left + (right - left) / 2) {
            if chs[k] != chs[right - k + left] {
                return false;
            }
        }

        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_680() {
        assert_eq!(Solution::valid_palindrome("abca".to_owned()), true);
    }
}
