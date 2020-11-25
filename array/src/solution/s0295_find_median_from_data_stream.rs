#![allow(unused)]

use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::cmp::Ordering;

struct MedianFinder {
  lo: BinaryHeap<i32>,
  hi: BinaryHeap<Reverse<i32>>, 
}

// https://leetcode.com/problems/find-median-from-data-stream/
// reference: https://leetcode.com/problems/find-median-from-data-stream/discuss/488917/Rust-20ms-10.9MB-beat-100
//
/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    /** initialize your data structure here. */
    fn new() -> Self {
      Self{
        lo: BinaryHeap::new(), 
        hi: BinaryHeap::new(),
      }
    }
    
    fn add_num(&mut self, num: i32) {
      match (self.lo.peek(), self.hi.peek()) {
        (None, Some(_)) => self.hi.push(Reverse(num)), 
        (None, None) => self.lo.push(num), 

        (Some(_), Some(Reverse(h))) if num >= *h => self.hi.push(Reverse(num)),
        (Some(_), Some(_)) => self.lo.push(num),
        (Some(_), None) => self.lo.push(num),
      };

      match self.lo.len().cmp(&self.hi.len()) {
        Ordering::Greater => {
          if let Some(l) = self.lo.pop() {
            self.hi.push(Reverse(l));
          }
        }
        Ordering::Less => {
          if let Some(Reverse(h)) = self.hi.pop() {
            self.lo.push(h);
          }
        }
        Ordering::Equal => {}
      }
    }
    
    fn find_median(&self) -> f64 {
       match (self.lo.peek(), self.hi.peek()) {
         (Some(left), Some(Reverse(right))) => {
           if (self.lo.len() + self.hi.len()) % 2 == 0 {
             (left + right) as f64 / 2.0 
           } else if self.lo.len() > self.hi.len() {
             *left as f64
           } else  {
             *right as f64
           }
         }
         (Some(l), None) => *l as f64,
         (None, Some(Reverse(h))) => *h as f64, 
         (None, None) => 0.0
       }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_239() {
      let mut mf = MedianFinder::new();
      mf.add_num(1);
      mf.add_num(2);
      assert_eq!(mf.find_median(), 1.5);
      mf.add_num(3);
      assert_eq!(mf.find_median(), 2.0);
    }
}
