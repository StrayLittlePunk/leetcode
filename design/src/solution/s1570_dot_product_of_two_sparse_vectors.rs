#![allow(unused)]

use std::collections::HashMap;
struct SparseVector {
    map: HashMap<usize, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SparseVector {
    fn new(nums: Vec<i32>) -> Self {
        let map = nums
            .into_iter()
            .enumerate()
            .skip_while(|(_, x)| *x == 0)
            .collect::<HashMap<usize, i32>>();
        Self { map }
    }

    // Return the dotProduct of two sparse vectors
    fn dot_product(&self, vec: SparseVector) -> i32 {
        let mut ret = 0;
        if vec.map.len() > self.map.len() {
            for (k, v) in vec.map.iter() {
                if let Some(&c) = self.map.get(k) {
                    ret += c * v;
                }
            }
        } else {
            for (k, v) in self.map.iter() {
                if let Some(&c) = vec.map.get(k) {
                    ret += c * v;
                }
            }
        }

        ret
    }
}

/**
 * Your SparseVector object will be instantiated and called as such:
 * let v1 = SparseVector::new(nums1);
 * let v2 = SparseVector::new(nums2);
 * let ans = v1.dot_product(v2);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1570() {}
}
