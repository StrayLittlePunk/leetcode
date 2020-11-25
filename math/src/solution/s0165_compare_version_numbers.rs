#![allow(unused)]
pub struct Solution {}

impl Solution {
  // Time O(N + M) Space O(N + M);
  // N is length of v1, M is length of v2
    pub fn compare_version(version1: String, version2: String) -> i32 {
      let mut v1: Vec<u32> = version1.split('.').map(|s| s.parse().unwrap()).collect();
      let mut v2: Vec<u32> = version2.split('.').map(|s| s.parse().unwrap()).collect();

      while v1.len() < v2.len(){
        v1.push(0);
      }
      while v1.len() > v2.len(){
        v2.push(0);
      }

      // compare two array 
      match v1.cmp(&v2) {
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Greater => 1,
      }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_165() {
        assert_eq!(
            Solution::compare_version("1.01".to_string(), "1.001".to_string(),),
            0
        );
        assert_eq!(
            Solution::compare_version("1.0".to_string(), "1.0.0".to_string(),),
            0
        );
        assert_eq!(
            Solution::compare_version("0.1".to_string(), "1.1".to_string(),),
            -1
        );
        assert_eq!(
            Solution::compare_version("1.0.1".to_string(), "1".to_string(),),
            1
        );
        assert_eq!(
            Solution::compare_version("7.5.2.4".to_string(), "7.5.3".to_string(),),
            -1
        );
    }
}
