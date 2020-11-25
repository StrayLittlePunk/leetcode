#![allow(unused)]

use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Default)]
struct Twitter {
    time: i32,
    tweets: HashMap<i32, Vec<(i32, i32)>>,
    fans: HashMap<i32, HashSet<i32>>,
}

const FEED_MAX_LEN: usize = 10;

impl Twitter {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Compose a new tweet. */
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        if !self.tweets.contains_key(&user_id) {
            self.tweets.insert(user_id, Default::default());
            self.follow(user_id, user_id);
        }
        self.tweets
            .get_mut(&user_id)
            .unwrap()
            .insert(0, (self.time, tweet_id));
        self.time -= 1;
    }

    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::<(i32, i32)>::new();
        if let Some(set) = self.fans.get(&user_id) {
            for f in set {
                if let Some(tweets) = self.tweets.get(f) {
                    for t in tweets {
                        if heap.len() < FEED_MAX_LEN {
                            heap.push(*t);
                        } else {
                            if t.0 >= heap.peek().unwrap().0 {
                                break;
                            } else {
                                heap.push(*t);
                                heap.pop();
                            }
                        }
                    }
                }
            }
        }
        let mut ans = vec![];
        while let Some(x) = heap.pop() {
            ans.insert(0, x.1);
        }
        ans
    }

    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.fans
            .entry(follower_id)
            .or_default()
            .insert(followee_id);
    }

    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if followee_id != follower_id {
            self.fans
                .entry(follower_id)
                .or_default()
                .remove(&followee_id);
        }
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_355() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5);
        assert_eq!(twitter.get_news_feed(1), vec![5]);
        twitter.follow(1, 2);
        twitter.post_tweet(2, 6);
        //        println!("{:?}", twitter);
        assert_eq!(twitter.get_news_feed(1), vec![6, 5]);
        twitter.unfollow(1, 2);
        assert_eq!(twitter.get_news_feed(1), vec![5]);
    }
}
