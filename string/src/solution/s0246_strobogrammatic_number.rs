#![allow(unused)]
pub struct Solution {}
use std::collections::HashSet;
impl Solution {
  // O(n) O(n)
    pub fn is_strobogrammatic(num: String) -> bool {
        let mut map = HashSet::new();
        map.insert("0");
        map.insert("1");
        map.insert("8");
        map.insert("00");
        map.insert("11");
        map.insert("88");
        map.insert("69");
        map.insert("96");

        let chs = num.chars().collect::<Vec<char>>();
        let (mut left, mut right) = (0, chs.len() - 1);
        while left <= right {
            let mut tmp = "".to_owned();
            tmp.push(chs[left]);
            tmp.push(chs[right]);
            if !map.contains(&tmp.as_str()) {
                return false;
            }
            left += 1;
            if right == 0 {
              break;
            }
            right -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_246() {
        assert_eq!(Solution::is_strobogrammatic("69".to_owned()), true);
        assert_eq!(Solution::is_strobogrammatic("88".to_owned()), true);
        assert_eq!(Solution::is_strobogrammatic("962".to_owned()), false);
        assert_eq!(Solution::is_strobogrammatic("1".to_owned()), true);
    }
}
