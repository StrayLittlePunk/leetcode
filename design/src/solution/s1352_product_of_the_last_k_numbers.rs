#![allow(unused)]

// apple interview
// prefix product
struct ProductOfNumbers {
    array: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl ProductOfNumbers {
    fn new() -> Self {
        Self { array: vec![1] }
    }

    fn add(&mut self, num: i32) {
        if num > 0 {
            let last = *self.array.last().unwrap();
            self.array.push(last * num);
        } else {
            // If we meet 0, the product including this 0 will always be 0.
            // We only need to record the prefix product after it.
            // So I clear the A and reinitilise it as [1],
            // where 1 is the neutral element of multiplication
            self.array = vec![1];
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let (n, k) = (self.array.len(), k as usize);
        if k < n {
            self.array[n - 1] / self.array[n - k - 1]
        } else {
            0
        }
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1352() {}
}
