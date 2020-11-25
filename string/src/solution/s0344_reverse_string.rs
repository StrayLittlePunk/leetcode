#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(n) O(1)
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.len() == 0 {
          return ;
        }

        let (mut lptr, mut rptr) = (0, s.len() - 1);
        let mut tmp;
        while lptr < rptr {
            tmp = s[lptr];
            s[lptr] = s[rptr];
            s[rptr] = tmp;

            lptr += 1;
            rptr -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_344() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);

        s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }
}
