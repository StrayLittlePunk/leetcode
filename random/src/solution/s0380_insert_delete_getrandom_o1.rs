#![allow(unused)]
use rand::{rngs::ThreadRng, thread_rng, Rng};

use std::collections::HashMap;
struct RandomizedSet {
    map: HashMap<i32, usize>,
    list: Vec<i32>,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            list: Vec::new(),
            rng: thread_rng(),
        }
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        if !self.map.contains_key(&val) {
            self.list.push(val);
            self.map.insert(val, self.list.len() - 1);
            return true;
        }
        false
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            let id = *self.map.get(&val).unwrap();
            let last = self.list[self.list.len() - 1];

            // swap last val to list[id], and update last val id in map
            self.list[id] = last;
            self.map.insert(last, id);

            // delete list and map the remove key-val
            self.list.pop();
            self.map.remove(&val);

            return true;
        }

        false
    }

    /** Get a random element from the set. */
    fn get_random(&mut self) -> i32 {
        // if list no elements , reutrn MIN mean a error occurs
        if self.list.len() < 1 {
          
            return std::i32::MIN;
        }
        let idx = self.rng.gen_range(0, self.list.len());
        self.list[idx]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_380() {
        let mut obj = RandomizedSet::new();
        assert_eq!(obj.insert(1), true);
        assert_eq!(obj.insert(2), true);
        assert_eq!(obj.insert(1), false);
        assert_eq!(obj.remove(1), true);
        assert_eq!(obj.remove(2), true);
        assert_eq!(obj.remove(2), false);
        assert_eq!(obj.get_random(), std::i32::MIN);
        assert_eq!(obj.insert(2), true);
        assert_eq!(obj.insert(3), true);
      //  assert_eq!(obj.get_random(), 3);
    }
}
