#![allow(unused)]


struct Vector2D {
    vector: Vec<i32>,
    cur: usize,
    len: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Vector2D {
    fn new(v: Vec<Vec<i32>>) -> Self {
        let len: usize = v.iter().map(|c| c.len()).sum();
        let mut ans = vec![];
        for vector in v.into_iter() {
            for n in vector {
                ans.push(n);
            }
        }

        Self {
            vector: ans,
            cur: 0,
            len: len,
        }
    }

    fn next(&mut self) -> i32 {
        if !self.has_next() {
            return -1;
        }

        let ret = self.vector[self.cur];
        self.cur += 1;

        return ret;
    }

    fn has_next(&self) -> bool {
        self.cur < self.len
    }
}

/**
 * Your Vector2D object will be instantiated and called as such:
 * let obj = Vector2D::new(v);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_346() {}
}
