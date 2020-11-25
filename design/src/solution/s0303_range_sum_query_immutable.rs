#![allow(unused)]

struct NumArray {
    sum: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            sum[i + 1] = sum[i] + nums[i];
        }

        Self { sum }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
      self.sum[j as usize + 1] - self.sum[i as usize]
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(i, j);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_346() {}
}
