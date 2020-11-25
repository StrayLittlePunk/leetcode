#![allow(unused)]

struct NumArray {
    tree: Vec<i32>,
    n: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        if nums.len() > 0 {
            let n = nums.len();
            let mut tree = vec![0; n * 2];
            for i in n..2 * n {
                tree[i] = nums[i - n];
            }
            for i in (0..n).rev() {
                tree[i] = tree[i * 2] + tree[i * 2 + 1];
            }

            return Self {
                tree,
                n: nums.len(),
            };
        }

        Self {
            tree: Vec::new(),
            n: 0,
        }
    }

    fn update(&mut self, i: i32, val: i32) {
        let mut i = i as usize;
        i += self.n;
        self.tree[i] = val;
        while i > 0 {
            let (mut left, mut right) = (i, i);
            if i % 2 == 0 {
                right = i + 1;
            } else {
                left = i - 1;
            }

            self.tree[i / 2] = self.tree[left] + self.tree[right];
            i /= 2;
        }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let (mut l, mut r, mut sum) = (i as usize + self.n, j as usize + self.n, 0);
        while l <= r {
            if (l % 2) == 1 {
                sum += self.tree[l];
                l += 1;
            }

            if (r % 2) == 0 {
                sum += self.tree[r];
                r -= 1;
            }

            l /= 2;
            r /= 2;
        }

        sum
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(i, val);
 * let ret_2: i32 = obj.sum_range(i, j);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_346() {}
}
