#![allow(unused)]

struct NumMatrix {
    tree: Vec<Vec<i32>>,
    nums: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return Self {
                tree: vec![],
                nums: vec![],
            };
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut tree = vec![vec![0; n]; m + 1];

        for i in 1..=m {
            for j in 0..n {
                tree[i][j] = tree[i - 1][j] + matrix[i - 1][j];
            }
        }

        Self {
            tree: tree,
            nums: matrix,
        }
    }

    fn update(&mut self, row: i32, col: i32, val: i32) {
        for i in (row as usize + 1)..self.tree.len() {
            self.tree[i][col as usize] =
                self.tree[i][col as usize] - self.nums[row as usize][col as usize] + val;
        }

        self.nums[row as usize][col as usize] = val;
    }

    fn sum_region(&mut self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut ret = 0;
        let (row1, col1, row2, col2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);

        for j in col1..=col2 {
            ret += self.tree[row2 + 1][j] - self.tree[row1][j];
        }

        ret
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * obj.update(row, col, val);
 * let ret_2: i32 = obj.sum_region(row1, col1, row2, col2);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_346() {}
}
