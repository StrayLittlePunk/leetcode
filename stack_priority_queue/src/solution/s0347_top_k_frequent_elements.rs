#![allow(unused)]
pub struct Solution {}

use std::collections::{HashMap,BinaryHeap};
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
pub struct Value {
    number: i32,
    occurrences: usize,
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        Some(other.occurrences.cmp(&self.occurrences))
    }
}


impl Ord for Value {
  fn cmp(&self, other: &Value) -> Ordering {
    other.occurrences.cmp(&self.occurrences)
  }
}

impl Value {
  pub fn new(number: i32, occurrences: usize) -> Self {
    Self{
      number, occurrences,
    }
  }
}

impl Solution {
    // Time O(Nlog k), Space O(N) use heap
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
      if k > nums.len() as i32{
        return vec![];
      }
      let mut map: HashMap<i32, usize> = HashMap::new();
      for num in nums {
            map.entry(num).and_modify(|v| *v += 1).or_insert(1);
      }

      let mut heap: BinaryHeap<Value> = BinaryHeap::with_capacity(k as usize);
      for (number, occurrences) in map.drain() {
        heap.push(Value::new(number, occurrences));
        if heap.len() > k as usize {
          heap.pop();
        }
      }


      let mut res: Vec<i32> = Vec::with_capacity(k as usize);
      for v in heap.drain() {
        res.push(v.number);
      }

      res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_347() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![2, 1]
        );
        assert_eq!(
            Solution::top_k_frequent(vec![1], 1),
            vec![1]
        );
    }
}
