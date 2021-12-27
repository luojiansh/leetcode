/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */
pub struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m: HashMap<&i32, usize> = nums.iter().enumerate().map(|x| (x.1, x.0)).collect();
        for (i, a) in nums.iter().enumerate() {
            if let Some(j) = m.get(&(target - a)) {
                if i != *j {
                    return vec![i as i32, *j as i32];
                }
            }
            m.insert(a, i);
        }
        vec![]
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
        assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6));
    }
}
