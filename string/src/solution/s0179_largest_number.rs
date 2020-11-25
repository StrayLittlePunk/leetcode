#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(n log n) O(n)
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut strs = nums
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        strs.sort_by(|a, b| format!("{}{}", b, a).cmp(&format!("{}{}", a, b)));

        let mut ret = strs.join("");
        if ret.trim_start_matches("0").is_empty() {
            return "0".to_string();
        }

        ret.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_179() {
        assert_eq!(Solution::largest_number(vec![10, 2]), "210".to_string());
        assert_eq!(
            Solution::largest_number(vec![3, 30, 34, 5, 9]),
            "9534330".to_string()
        );
        assert_eq!(Solution::largest_number(vec![1]), "1".to_string());
        assert_eq!(Solution::largest_number(vec![10]), "10".to_string());
    }
}
