#![allow(unused)]

use std::collections::HashMap;
struct Logger {
    map: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Logger {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /** Returns true if the message should be printed in the given timestamp, otherwise returns false.
    If this method returns false, the message will not be printed.
    The timestamp is in seconds granularity. */
    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        if !self.map.contains_key(&message) {
            self.map.insert(message, timestamp);
            return true;
        }
        if timestamp - *self.map.get(&message).unwrap() >= 10 {
            if let Some(x) = self.map.get_mut(&message) {
                *x = timestamp;
            }
            return true;
        } else {
            return false;
        }
    }
}

/**
 * Your Logger object will be instantiated and called as such:
 * let obj = Logger::new();
 * let ret_1: bool = obj.should_print_message(timestamp, message);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_89() {
        let mut obj = Logger::new();
        let ret_1: bool = obj.should_print_message(1, "foo".to_owned());
        assert_eq!(ret_1, true);
    }
}
