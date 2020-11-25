#![allow(unused)]

use std::collections::{HashSet, VecDeque};

struct SnakeGame {
    set: HashSet<i32>,
    body: VecDeque<i32>,
    score: i32,
    food: Vec<Vec<i32>>,
    food_idx: i32,
    width: i32,
    height: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnakeGame {
    /** Initialize your data structure here.
    @param width - screen width
    @param height - screen height
    @param food - A list of food positions
    E.g food = [[1,1], [1,0]] means the first food is positioned at [1,1], the second is at [1,0]. */
    fn new(width: i32, height: i32, food: Vec<Vec<i32>>) -> Self {
        let mut set = HashSet::new();
        // initally at [0][0]
        set.insert(0);
        let mut queue = VecDeque::new();
        queue.push_back(0);
        Self {
            set: set,
            body: queue,
            score: 0,
            food: food,
            food_idx: 0,
            width: width,
            height: height,
        }
    }

    /** Moves the snake.
    @param direction - 'U' = Up, 'L' = Left, 'R' = Right, 'D' = Down
    @return The game's score after the move. Return -1 if game over.
    Game over when snake crosses the screen boundary or bites its body. */
    fn make_a_move(&mut self, direction: String) -> i32 {
        // case 0: game already over: do nothing
        if self.score == -1 {
            return -1;
        }

        // compute new head

        let mut row_head = *self.body.front().unwrap() / self.width;
        let mut col_head = *self.body.front().unwrap() % self.width;
        match direction.as_str() {
            "U" => {
                row_head -= 1;
            }
            "D" => {
                row_head += 1;
            }
            "L" => {
                col_head -= 1;
            }
            _ => {
                col_head += 1;
            }
        }

        let head = row_head * self.width + col_head;
        // case 1 out of boundary or eating body
        self.set.remove(self.body.back().unwrap());
        if row_head < 0
            || row_head == self.height
            || col_head < 0
            || col_head == self.width
            || self.set.contains(&head)
        {
            self.score = -1;
            return -1;
        }

        // add head for case2 and case3
        self.set.insert(head);
        self.body.push_front(head);

        // case2 eating food, keep tail, add head
        if self.food_idx < self.food.len() as i32
            && row_head == self.food[self.food_idx as usize][0]
            && col_head == self.food[self.food_idx as usize][1]
        {
            self.set.insert(*self.body.back().unwrap());
            self.food_idx += 1;
            self.score += 1;
            return self.score;
        }

        // case3 normal move, remove tail, add head
        self.body.pop_back();
        return self.score;
    }
}

/**
 * Your SnakeGame object will be instantiated and called as such:
 * let obj = SnakeGame::new(width, height, food);
 * let ret_1: i32 = obj.make_a_move(direction);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_346() {}
}
