/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 *
 * https://leetcode.com/problems/reverse-integer/description/
 *
 * algorithms
 * Medium (26.34%)
 * Likes:    6230
 * Dislikes: 9030
 * Total Accepted:    1.9M
 * Total Submissions: 7.2M
 * Testcase Example:  '123'
 *
 * Given a signed 32-bit integer x, return x with its digits reversed. If
 * reversing x causes the value to go outside the signed 32-bit integer range
 * [-2^31, 2^31 - 1], then return 0.
 *
 * Assume the environment does not allow you to store 64-bit integers (signed
 * or unsigned).
 *
 *
 * Example 1:
 *
 *
 * Input: x = 123
 * Output: 321
 *
 *
 * Example 2:
 *
 *
 * Input: x = -123
 * Output: -321
 *
 *
 * Example 3:
 *
 *
 * Input: x = 120
 * Output: 21
 *
 *
 *
 * Constraints:
 *
 *
 * -2^31 <= x <= 2^31 - 1
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut y = 0 as i32;
        while x != 0 {
            let t = x % 10;
            x /= 10;
            match y.checked_mul(10) {
                Some(v) => match v.checked_add(t) {
                    Some(v) => y = v,
                    None => {
                        y = 0;
                        break;
                    }
                },
                None => {
                    y = 0;
                    break;
                }
            }
        }
        y
    }
}
// @lc code=end
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        for (i, o) in [(123, 321), (-123, -321), (120, 21)] {
            assert_eq!(Solution::reverse(i), o);
        }
    }
}
