#![allow(unused)]

use std::collections::{HashMap, VecDeque};

// amazon interview
struct FirstUnique {
    queue: VecDeque<i32>,
    map: HashMap<i32, usize>,
}

impl FirstUnique {
    // O(n)
    fn new(nums: Vec<i32>) -> Self {
        // create frequency map
        let mut map = HashMap::new();
        for &num in nums.iter() {
            *map.entry(num).or_insert(0) += 1;
        }

        let queue = nums.into_iter().collect();
        Self { queue, map }
    }
    // O(1) amortized
    // For this implementation, the showFirstUnique() method needs to iterate down the queue until it finds a unique
    // number. For each unique number it encounters along the way, it removes it. Removing an item from a queue has a
    // cost of O(1). The total number of these removals we need to carry out is proportional to the number of calls
    // to add(), because each add() corresponds to at most one removal that will ultimately have to happen. Then when w
    // find a unique number, it is an O(1) operation to return it.

    // Because the number of O(1) removals is proportional to the number of calls to add(), we say that the time
    // complexity amortizes across all calls to add() and showFirstUnique(), giving an overall time complexity of
    // O(1) (amortized).
    fn show_first_unique(&mut self) -> i32 {
        // found duplicate value will pop , and unique value stay in queue,
        // absctract queue will store all duplicate value, so don't need to map.get(&val) -= 1,
        // jusify map.get(val) >= 1 mean absctract will have this duplicate val
         while let Some(val) = self.queue.pop_front() {
            if *self.map.get(&val).unwrap() == 1 {
                self.queue.push_front(val);
                return val;
            }
        }
        -1
    }
    // O(1)
    fn add(&mut self, value: i32) {
        self.queue.push_back(value);
        *self.map.entry(value).or_insert(0) += 1;
    }
}


/**
 * Your FirstUnique object will be instantiated and called as such:
 * let obj = FirstUnique::new(nums);
 * let ret_1: i32 = obj.show_first_unique();
 * obj.add(value);
 */


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1429() {}
}
