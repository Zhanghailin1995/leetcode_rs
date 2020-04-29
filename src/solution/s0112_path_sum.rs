/**
 * [112] Path Sum
 *
 * Given a binary tree and a sum, determine if the tree has a root-to-leaf path such that adding up all the values along the path equals the given sum.
 *
 * Note: A leaf is a node with no children.
 *
 * Example:
 *
 * Given the below binary tree and sum = 22,
 *
 *
 *       5
 *      / \
 *     4   8
 *    /   / \
 *   11  13  4
 *  /  \      \
 * 7    2      1
 *
 *
 * return true, as there exist a root-to-leaf path 5->4->11->2 which sum is 22.
 *
 */
pub struct Solution {}

use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/path-sum/
// discuss: https://leetcode.com/problems/path-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        let node = root.as_ref().unwrap().borrow();
        match (node.left.clone(), node.right.clone()) {
            (None, None) => sum == node.val,
            (l, r) => {
                Self::has_path_sum(l, sum - node.val)
                    || Self::has_path_sum(r, sum - node.val)
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_112() {
        assert_eq!(
            Solution::has_path_sum(
                tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1],
                22,
            ),
            true
        );
    }
}
