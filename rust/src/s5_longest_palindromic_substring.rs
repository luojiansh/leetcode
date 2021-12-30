/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 *
 * https://leetcode.com/problems/longest-palindromic-substring/description/
 *
 * algorithms
 * Medium (31.56%)
 * Likes:    14889
 * Dislikes: 877
 * Total Accepted:    1.6M
 * Total Submissions: 5.1M
 * Testcase Example:  '"babad"'
 *
 * Given a string s, return the longest palindromic substring in s.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "babad"
 * Output: "bab"
 * Explanation: "aba" is also a valid answer.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "cbbd"
 * Output: "bb"
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 1000
 * s consist of only digits and English letters.
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut longest = (0, 0);
        let bytes = s.as_bytes();
        let total = bytes.len() as i32;
        for j in 2..total * 2 {
            let i = j / 2;
            let mut forward = i + j % 2;
            let mut backward = i - 1;
            let mut start = i;
            let mut end = i;
            while forward < total
                && backward >= 0
                && bytes[backward as usize] == bytes[forward as usize]
            {
                start = backward;
                backward -= 1;
                end = forward;
                forward += 1;
            }
            if end - start > longest.1 - longest.0 {
                longest = (start, end)
            }
        }
        s[longest.0 as usize..=longest.1 as usize].to_string()
    }
}
// @lc code=end

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        for (input, output) in [
            ("aacabdkacaa", "aca"),
            ("ccc", "ccc"),
            ("bababd", "babab"),
            ("cbbd", "bb"),
        ] {
            assert_eq!(Solution::longest_palindrome(input.to_string()), output)
        }
    }
}
