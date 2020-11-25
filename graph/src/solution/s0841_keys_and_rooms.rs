#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        if rooms.is_empty() {
            return false;
        }
        let mut seen = vec![false; rooms.len()];
        seen[0] = true;
        let mut stack = vec![];
        stack.push(0);

        //At the beginning, we have a todo list "stack" of keys to use.
        //'seen' represents at some point we have entered this room.
        while let Some(node) = stack.pop() {
            // While we have keys...Get the next key 'node'
            for k in rooms[node].iter() {
                // For every key in room # 'node'...
                if !seen[*k as usize] {
                    // ...that hasn't been used yet
                    seen[*k as usize] = true; // mark that we've entered the room
                    stack.push(*k as usize); // add the key to the todo list
                }
            }
        }

        for b in seen {
            // if any room hasn't been visited, return false
            if !b {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_841() {
        assert_eq!(
            Solution::can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![],]),
            true
        );
        assert_eq!(
            Solution::can_visit_all_rooms(vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0],]),
            false
        );
    }
}
