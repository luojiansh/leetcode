/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 */

pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// @lc code=start
// helper function for test
pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut r1 = &l1;
        let mut r2 = &l2;
        let mut v3 = vec![];
        loop {
            let v;
            match (r1, r2) {
                (None, None) => {
                    if carry > 0 {
                        v3.push(carry);
                    }
                    break;
                }
                (Some(n1), Some(n2)) => {
                    v = n1.val + n2.val + carry;
                    r1 = &n1.next;
                    r2 = &n2.next;
                }
                (Some(n1), None) => {
                    v = n1.val + carry;
                    r1 = &n1.next;
                }
                (None, Some(n2)) => {
                    v = n2.val + carry;
                    r2 = &n2.next
                }
            }
            v3.push(v % 10);
            carry = (v >= 10) as i32;
        }
        to_list(v3)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        for (v1, v2, expect) in [
            (
                vec![9],
                vec![1, 9, 9, 9, 9, 9, 9, 9, 9, 9],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            ),
            (vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]),
            (vec![0], vec![0], vec![0]),
            (
                vec![9, 9, 9, 9, 9, 9, 9],
                vec![9, 9, 9, 9],
                vec![8, 9, 9, 9, 0, 0, 0, 1],
            ),
        ]
        .into_iter()
        {
            let mut l3 = vec![];
            let mut out = Solution::add_two_numbers(to_list(v1), to_list(v2));
            while let Some(node) = out {
                l3.push(node.val);
                out = node.next;
            }
            assert_eq!(l3, expect);
        }
    }
}
