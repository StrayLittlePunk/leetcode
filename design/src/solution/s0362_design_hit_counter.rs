#![allow(unused)]

struct HitCounter {
    hit_time: Vec<i32>,
    hit_count: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl HitCounter {
    const NUM_TS: i32 = 300;

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            hit_time: vec![0; Self::NUM_TS as usize],
            hit_count: vec![0; Self::NUM_TS as usize],
        }
    }

    /** Record a hit.
    @param timestamp - The current timestamp (in seconds granularity). */
    fn hit(&mut self, timestamp: i32) {
        let idx = (timestamp % Self::NUM_TS) as usize;
        if self.hit_time[idx] != timestamp {
            self.hit_time[idx] = timestamp;
            self.hit_count[idx] = 1;
        } else {
            self.hit_count[idx] += 1;
        }
    }

    /** Return the number of hits in the past 5 minutes.
    @param timestamp - The current timestamp (in seconds granularity). */
    fn get_hits(&self, timestamp: i32) -> i32 {
        let mut ret = 0;
        for i in 0..Self::NUM_TS {
            if timestamp - self.hit_time[i as usize] < Self::NUM_TS {
                ret += self.hit_count[i as usize];
            }
        }

        ret
    }
}

/**
 * Your HitCounter object will be instantiated and called as such:
 * let obj = HitCounter::new();
 * obj.hit(timestamp);
 * let ret_2: i32 = obj.get_hits(timestamp);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_346() {}
}
