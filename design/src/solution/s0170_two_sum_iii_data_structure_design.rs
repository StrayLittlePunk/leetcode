#![allow(unused)]

use std::collections::HashMap;
struct TwoSum {
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TwoSum {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self { nums: vec![] }
    }

    /** Add the number to an internal data structure.. */
    fn add(&mut self, number: i32) {
        self.nums.push(number);
    }

    /** Find if there exists any pair of numbers which sum is equal to the value. */
    fn find(&self, value: i32) -> bool {
        let mut map = HashMap::new();
        for n in self.nums.iter() {
            if !map.contains_key(n) {
                map.insert(value - *n, n);
            } else {
              return true;
            } 
        }

        false
    }
}

/**
 * Your TwoSum object will be instantiated and called as such:
 * let obj = TwoSum::new();
 * obj.add(number);
 * let ret_2: bool = obj.find(value);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_346() {}
}
