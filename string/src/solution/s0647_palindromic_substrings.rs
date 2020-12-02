#![allow(unused)]
pub struct Solution {}

// microsoft interview
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let (n, mut ans) = (s.len(), 0);
        for center in 0..n * 2 {
            let mut left = center / 2;
            let mut right = left + center % 2;
            while right < n && s[left] == s[right] {
                ans += 1;
                if left == 0 {
                    break;
                }
                left -= 1;
                right += 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_647() {}
}
