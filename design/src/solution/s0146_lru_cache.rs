#![allow(unused)]

use std::convert::TryInto;
use std::collections::VecDeque;
use std::collections::HashMap;


struct LRUCache {
    dq: VecDeque<i32>,
    map: HashMap<i32, i32>,
    cap: usize,
}

impl LRUCache {

    fn new(capacity: i32) -> Self {
        Self {
            dq: VecDeque::new(),
            map: HashMap::new(),
            cap: capacity.try_into().unwrap(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.map.contains_key(&key) {
          let i = self.dq.iter().position(|&x| x == key).unwrap();
          self.dq.remove(i);
          self.dq.push_front(key);
          return self.map.get(&key).cloned().unwrap();
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            let i = self.dq.iter().position(|&x| x == key).unwrap();
            self.dq.remove(i);
            self.dq.push_front(key);
            self.map.insert(key, value);
        } else {
            if self.map.len() == self.cap {
                let last = self.dq.pop_back().unwrap();
                self.map.remove(&last);
            }
            self.map.insert(key, value);
            self.dq.push_front(key);
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_346() {}
}
