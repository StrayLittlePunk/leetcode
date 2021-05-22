#![allow(unused)]
pub struct Solution {}

enum Direction {
    Top,
    Down,
    Left,
    Right,
}

use std::cmp::{max, min};
impl Solution {
    pub fn min_area_bf(image: Vec<Vec<char>>, x: i32, y: i32) -> i32 {
        let (mut top, mut bottom, mut left, mut right) = (x, x, y, y);
        for x in 0..image.len() {
            for y in 0..image[0].len() {
                if image[x][y] == '1' {
                    top = min(top, x as i32);
                    bottom = max(bottom, x as i32 + 1);
                    left = min(left, y as i32);
                    right = max(right, y as i32 + 1);
                }
            }
        }

        (right - left) * (bottom - top)
    }

    pub fn min_area(image: Vec<Vec<char>>, x: i32, y: i32) -> i32 {
        let top = Self::find(&image, 0, x as usize, 0, image[0].len() - 1, Direction::Top);
        let down = Self::find(
            &image,
            x as usize,
            image.len() - 1,
            0,
            image[0].len() - 1,
            Direction::Down,
        );
        let left = Self::find(
            &image,
            0,
            y as usize,
            top as usize,
            down as usize,
            Direction::Left,
        );
        let right = Self::find(
            &image,
            y as usize,
            image[0].len() - 1,
            top as usize,
            down as usize,
            Direction::Right,
        );

        (right - left + 1) * (down - top + 1)
    }

    fn find(
        image: &Vec<Vec<char>>,
        mut start: usize,
        mut end: usize,
        lo: usize,
        hi: usize,
        direct: Direction,
    ) -> i32 {
        while start + 1 < end {
            let mid = start + (end - start) / 2;

            if match direct {
                Direction::Top | Direction::Down => image[mid][lo..=hi].iter().any(|&c| c == '1'),
                Direction::Left | Direction::Right => {
                    image[lo..=hi].iter().any(|arr| arr[mid] == '1')
                }
            } {
                match direct {
                    Direction::Top | Direction::Left => {
                        end = mid;
                    },
                    Direction::Down | Direction::Right => {
                        start = mid;
                    }
                }
            } else {
                match direct {
                    Direction::Top | Direction::Left => {
                        start = mid;
                    },
                    Direction::Down | Direction::Right => {
                        end = mid;
                    }
                }
            }
        }

        if match direct {
            Direction::Top => image[start][lo..=hi].iter().any(|&c| c == '1'),
            Direction::Down => image[end][lo..=hi].iter().any(|&c| c == '1'),
            Direction::Left => image[lo..=hi].iter().any(|arr| arr[start] == '1'),
            Direction::Right => image[lo..=hi].iter().any(|arr| arr[end] == '1'),
        } {
            match direct {
                Direction::Top | Direction::Left => start as i32,
                Direction::Down | Direction::Right => end as i32,
            }
        } else {
            match direct {
                Direction::Top | Direction::Left => end as i32,
                Direction::Down | Direction::Right => start as i32,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_302() {
        assert_eq!(
            Solution::min_area(
                vec![
                    vec!['0', '0', '1', '0'],
                    vec!['0', '1', '1', '0'],
                    vec!['0', '1', '0', '0']
                ],
                0,
                2
            ),
            6
        );
        assert_eq!(
            Solution::min_area(
                vec![
                    vec!['1', '1', '1', '0'],
                    vec!['1', '1', '0', '0'],
                    vec!['0', '0', '0', '0'],
                    vec!['0', '0', '0', '0']
                ],
                0,
                1
            ),
            6
        );
    }
}
