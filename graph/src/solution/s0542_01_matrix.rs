pub struct Solution;

impl Solution {
    pub fn update_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;

        let dir = [[1, 0], [0, 1], [-1, 0], [0, -1]];
        let row = matrix.len();
        if row < 1 {
            return matrix;
        };
        let col = matrix[0].len();
        let mut queue = VecDeque::<(i32, i32)>::new();
        for i in 0..row as i32 {
            for j in 0..col as i32 {
                if matrix[i as usize][j as usize] == 0 {
                    queue.push_back((i, j))
                } else {
                    matrix[i as usize][j as usize] = std::i32::MAX;
                }
            }
        }
        while !queue.is_empty() {
            let (i, j) = queue.pop_front().unwrap();
            for count in 0..4 {
                let y = i + dir[count][1];
                let x = j + dir[count][0];
                if x >= 0 && x < col as i32 && y >= 0 && y < row as i32 {
                    let num = matrix[i as usize][j as usize] + 1;
                    if matrix[y as usize][x as usize] > num {
                        matrix[y as usize][x as usize] = num;
                        queue.push_back((y, x));
                    }
                }
            }
        }
        matrix
    }
}
