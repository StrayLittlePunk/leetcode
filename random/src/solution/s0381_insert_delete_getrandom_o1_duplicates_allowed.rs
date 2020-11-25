#![allow(unused)]
use rand::{rngs::ThreadRng, thread_rng, Rng};

use std::collections::{HashMap, HashSet};
struct RandomizedCollection {
    map: HashMap<i32, HashSet<usize>>,
    list: Vec<i32>,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {
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
        let set = self.map.entry(val).or_insert(HashSet::new());
        // insert duplicate or one val
        set.insert(self.list.len());
        self.list.push(val);

        // check duplicate
        set.len() == 1
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        if !self.map.contains_key(&val) {
            return false;
        }


        let id = *self.map.get(&val).unwrap().iter().next().unwrap();
        self.map.get_mut(&val).unwrap().remove(&id);

        let last = self.list[self.list.len() - 1];

        // swap last val to list[id], and update last val id in map
        self.list[id] = last;
        self.map.get_mut(&last).unwrap().insert(id);
        self.map
            .get_mut(&last)
            .unwrap()
            .remove(&(self.list.len() - 1));

        if self.map.get(&val).unwrap().len() == 0 {
            self.map.remove(&val);
        }

        self.list.pop();
        true
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
    // for test 
    fn get(&self, val: i32) -> bool {
      self.map.contains_key(&val)
    }
}

/**
 * Your RandomizedCollection object will be instantiated and called as such:
 * let obj = RandomizedCollection::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_381() {
        let mut obj = RandomizedCollection::new();
        assert_eq!(obj.insert(1), true);
        assert_eq!(obj.insert(1), false);
        assert_eq!(obj.insert(2), true);
        assert_eq!(obj.insert(1), false);
        assert_eq!(obj.remove(1), true);
        assert_eq!(obj.remove(1), true);
        assert_eq!(obj.remove(2), true);
        assert_eq!(obj.remove(2), false);
        assert_eq!(obj.get_random(), 1);
        assert_eq!(obj.remove(1), true);
        assert_eq!(obj.get_random(), std::i32::MIN);
        assert_eq!(obj.insert(2), true);
        assert_eq!(obj.insert(3), true);
        assert_eq!(obj.get(1), false);
        //assert_eq!(obj.get_random(), 3);
    }
}
