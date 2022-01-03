/*
 * @lc app=leetcode id=6 lang=rust
 *
 * [6] Zigzag Conversion
 *
 * https://leetcode.com/problems/zigzag-conversion/description/
 *
 * algorithms
 * Medium (40.47%)
 * Likes:    3137
 * Dislikes: 7389
 * Total Accepted:    684.9K
 * Total Submissions: 1.7M
 * Testcase Example:  '"PAYPALISHIRING"\n3'
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number
 * of rows like this: (you may want to display this pattern in a fixed font for
 * better legibility)
 *
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 *
 *
 * And then read line by line: "PAHNAPLSIIGYIR"
 *
 * Write the code that will take a string and make this conversion given a
 * number of rows:
 *
 *
 * string convert(string s, int numRows);
 *
 *
 *
 * Example 1:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 *
 *
 * Example 2:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 *
 *
 * Example 3:
 *
 *
 * Input: s = "A", numRows = 1
 * Output: "A"
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 1000
 * s consists of English letters (lower-case and upper-case), ',' and '.'.
 * 1 <= numRows <= 1000
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows;
        let mut buf = vec![vec![]; num_rows as usize];
        let mut row = 0;
        let mut inc = if num_rows == 1 { 0 } else { -1 };
        for &b in s.as_bytes().iter() {
            buf[row as usize].push(b);
            if row == 0 || row == num_rows - 1 {
                inc = -inc;
            }
            row += inc;
        }
        let result = buf
            .into_iter()
            .map(|row| String::from_utf8(row).unwrap())
            .collect();
        result
    }
}
// @lc code=end

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        for (s, num_rows, expect) in [
            ("AB", 1, "AB"),
            ("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR"),
            ("PAYPALISHIRING", 4, "PINALSIGYAHRPI"),
            ("A", 1, "A"),
        ] {
            assert_eq!(Solution::convert(s.to_string(), num_rows), expect);
        }
    }
}
