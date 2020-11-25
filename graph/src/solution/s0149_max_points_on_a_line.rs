#![allow(unused)]
pub struct Solution {}

impl Solution {
    // Time O(n^2) Space O(N)
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        use std::cmp::max;
        use std::collections::HashMap;
        use std::i32::MAX;

        fn max_points_on_a_line_containing_point_i(
            points: &Vec<Vec<i32>>,
            i: usize,
            n: usize,
        ) -> i32 {
            fn slope_coprime(x1: i32, y1: i32, x2: i32, y2: i32) -> (i32, i32) {
                let (mut delta_x, mut delta_y) = (x1 - x2, y1 - y2);
                if delta_x == 0 {
                    return (0, 0);
                } else if delta_y == 0 {
                    return (MAX, MAX);
                } else if delta_x < 0 {
                    delta_x = -delta_x;
                    delta_y = -delta_y;
                }
                let gcd = Solution::gcd(delta_x , delta_y);
                (delta_x / gcd, delta_y / gcd )
            }

            fn add_line(
                lines_map: &mut HashMap<(i32, i32), i32>,
                points: &Vec<Vec<i32>>,
                i: usize,
                j: usize,
                mut count: i32,
                mut duplicates: i32,
                horizontal_lines: &mut i32,
            ) -> (i32, i32) {
                let (x1, y1, x2, y2) = (points[i][0], points[i][1], points[j][0], points[j][1]);

                if x1 == x2 && y1 == y2 {
                    duplicates += 1;
                } else if y1 == y2 {
                    *horizontal_lines += 1;
                    count = max(*horizontal_lines, count);
                } else {
                    let slope = slope_coprime(x1, y1, x2, y2);
                    let t = *lines_map.entry(slope).or_insert(1);
                    lines_map.insert(slope, t + 1);
                    count = max(*lines_map.get(&slope).unwrap(), count);
                }

                (count, duplicates)
            }
            //  init lines passing through point i
            let mut horizontal_lines = 1;

            let mut map = HashMap::new();
            // One starts with just one point on a line : point i.
            let mut counts = (1, 0);
            // Compute lines passing through point i (fixed)
            // and point j (interation).
            // Update in a loop the number of points on a line
            //  and the number of duplicates of point i.
            for j in (i + 1)..n {
                counts = add_line(
                    &mut map,
                    points,
                    i,
                    j,
                    counts.0,
                    counts.1,
                    &mut horizontal_lines,
                );
            }
            counts.0 + counts.1
        }

        let n = points.len();
        if n < 3 {
            return n as i32;
        }
        let mut max_count = 1;
        // Compute in a loop a max number of points
        // on a line containing point i.
        for i in 0..(n - 1) {
            max_count = max(
                max_points_on_a_line_containing_point_i(&points, i, n),
                max_count,
            )
        }
        max_count
    }

    pub fn gcd(u: i32, v: i32) -> i32{
      if v == 0 {
         u
      } else {
        Self::gcd(v, u % v)
      }
    }

    pub fn binary_gcd(mut u: u64, mut v: u64) -> u64 {
        use std::cmp::min;
        use std::mem::swap;

        // Base cases: gcd(n, 0) = gcd(0, n) = n
        if u == 0 {
            return v;
        } else if v == 0 {
            return u;
        }

        // Using identities 2 and 3:
        // gcd(2ⁱ u, 2ʲ v) = 2ᵏ gcd(u, v) with u, v odd and k = min(i, j)
        // 2ᵏ is the greatest power of two that divides both u and v
        let i = u.trailing_zeros();
        u >>= i;
        let j = v.trailing_zeros();
        v >>= j;
        let k = min(i, j);

        loop {
            // u and v are odd at the start of the loop
            debug_assert!(u % 2 == 1, "u = {} is even", u);
            debug_assert!(v % 2 == 1, "v = {} is even", v);

            // Swap if necessary so u <= v
            if u > v {
                swap(&mut u, &mut v);
            }

            // Using identity 4 (gcd(u, v) = gcd(|v-u|, min(u, v))
            v -= u;

            if v == 0 {
                return u << k;
            }

            // Identity 3: gcd(u, 2ʲ v) = gcd(u, v) (u is known to be odd)
            v >>= v.trailing_zeros();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_149() {
        // code here
        assert_eq!(
            Solution::max_points(vec![
                vec![1, 1],
                vec![3, 2],
                vec![5, 3],
                vec![4, 1],
                vec![2, 3],
                vec![1, 4]
            ]),
            4
        );
        assert_eq!(
            Solution::max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            3
        );
    }
}
