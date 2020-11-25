#![allow(unused)]

struct PhoneDirectory {
    next: Vec<i32>,
    pos: usize,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl PhoneDirectory {
    /** Initialize your data structure here
    @param maxNumbers - The maximum numbers that can be stored in the phone directory. */
    fn new(max_numbers: i32) -> Self {
        let mut next = vec![];
        for i in 0..max_numbers {
            next.push((i + 1) % max_numbers);
        }
        Self { next, pos: 0 }
    }

    /** Provide a number which is not assigned to anyone.
    @return - Return an available number. Return -1 if none is available. */
    fn get(&mut self) -> i32 {
        if self.next[self.pos] == -1 {
            return -1;
        }
        let ret = self.pos;
        self.pos = self.next[self.pos] as usize;
        self.next[self.pos] = -1;
        return ret as i32;
    }

    /** Check if a number is available or not. */
    fn check(&self, number: i32) -> bool {
        self.next[number as usize] == -1
    }

    /** Recycle or release a number. */
    fn release(&mut self, number: i32) {
        if self.next[number as usize] != -1 {
            return;
        }
        self.next[number as usize] = self.pos as i32;
        self.pos = number as usize;
    }
}

/**
 * Your PhoneDirectory object will be instantiated and called as such:
 * let obj = PhoneDirectory::new(maxNumbers);
 * let ret_1: i32 = obj.get();
 * let ret_2: bool = obj.check(number);
 * obj.release(number);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_346() {}
}
