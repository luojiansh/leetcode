/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

pub struct Solution {}

// @lc code=start

use std::collections::HashMap;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;
        let mut map = HashMap::with_capacity(128);
        let mut start = 0;
        for (i, c) in s.bytes().enumerate() {
            let j = i + 1;
            if let Some(&index) = map.get(&c) {
                if start < index {
                    start = index;
                }
            }
            let stride = j - start;
            max = if stride > max { stride } else { max };
            map.insert(c, j);
        }
        max as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        for (s, count) in [
            ("aab", 2),
            ("cdd", 2),
            ("abcabcbb", 3),
            ("bbbbb", 1),
            ("pwwkew", 3),
        ] {
            assert_eq!(Solution::length_of_longest_substring(s.to_string()), count)
        }
    }
}
