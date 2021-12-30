/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 *
 * https://leetcode.com/problems/median-of-two-sorted-arrays/description/
 *
 * algorithms
 * Hard (33.27%)
 * Likes:    13950
 * Dislikes: 1797
 * Total Accepted:    1.2M
 * Total Submissions: 3.6M
 * Testcase Example:  '[1,3]\n[2]'
 *
 * Given two sorted arrays nums1 and nums2 of size m and n respectively, return
 * the median of the two sorted arrays.
 *
 * The overall run time complexity should be O(log (m+n)).
 *
 *
 * Example 1:
 *
 *
 * Input: nums1 = [1,3], nums2 = [2]
 * Output: 2.00000
 * Explanation: merged array = [1,2,3] and median is 2.
 *
 *
 * Example 2:
 *
 *
 * Input: nums1 = [1,2], nums2 = [3,4]
 * Output: 2.50000
 * Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
 *
 *
 *
 * Constraints:
 *
 *
 * nums1.length == m
 * nums2.length == n
 * 0 <= m <= 1000
 * 0 <= n <= 1000
 * 1 <= m + n <= 2000
 * -10^6 <= nums1[i], nums2[i] <= 10^6
 *
 *
 */
// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len = nums1.len() + nums2.len();
        let half = len / 2;
        let mut seq1 = nums1.into_iter();
        let mut seq2 = nums2.into_iter();
        let mut val1 = seq1.next();
        let mut val2 = seq2.next();
        let mut median2 = 0;
        let mut median1 = 0;
        for _ in 0..=half {
            median1 = median2;
            match (val1, val2) {
                (Some(n1), Some(n2)) if n1 < n2 => {
                    median2 = n1;
                    val1 = seq1.next();
                }
                (Some(_), Some(n2)) => {
                    median2 = n2;
                    val2 = seq2.next();
                }
                (Some(n1), _) => {
                    median2 = n1;
                    val1 = seq1.next();
                }
                (_, Some(n2)) => {
                    median2 = n2;
                    val2 = seq2.next();
                }
                _ => panic!("can't reach this arm"),
            }
        }
        if len % 2 == 0 {
            (median2 + median1) as f64 / 2.0
        } else {
            median2 as f64
        }
    }
}
// @lc code=end

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        for (nums1, nums2, expect) in [
            (vec![2, 2, 4, 4], vec![2, 2, 4, 4], 3.0),
            (vec![1, 3], vec![2], 2.0),
            (vec![1, 2], vec![3, 4], 2.5),
        ]
        .into_iter()
        {
            assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expect);
        }
    }
}
