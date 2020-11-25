#![allow(unused)]

// google interview
use std::collections::BTreeMap;
struct MyCalendarTwo {
    delta: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {
    fn new() -> Self {
        Self {
            delta: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        *self.delta.entry(start).or_insert(0) += 1;
        *self.delta.entry(end).or_insert(0) -= 1;

        let mut active = 0;
        for d in self.delta.values() {
            active += d;
            if active >= 3 {
                self.delta
                    .insert(start, *self.delta.get(&start).unwrap() - 1);
                self.delta.insert(end, *self.delta.get(&end).unwrap() + 1);
                if *self.delta.get(&start).unwrap() == 0 {
                    self.delta.remove(&start);
                }
                return false;
            }
        }
        return true;
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(start, end);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_731() {}
}
