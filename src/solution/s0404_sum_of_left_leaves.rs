///404. 左叶子之和
/// 计算给定二叉树的所有左叶子之和。
///
/// 示例：
///
///     3
///    / \
///   9  20
///     /  \
///    15   7
///
/// 在这个二叉树中，有两个左叶子，分别是 9 和 15，所以返回 24

pub struct Solution {}

use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/binary-tree-paths/
// discuss: https://leetcode.com/problems/binary-tree-paths/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut sum = 0;
        let tree_node = root.as_ref().unwrap().borrow();
        //判断节点的左节点是否是叶子节点，如果是则将它的和累计起来
        if let Some(left) = tree_node.left.clone() {
            if let (None, None) = (left.borrow().left.clone(), left.borrow().right.clone()) {
                sum += left.borrow().val;
            }
        }
        Solution::sum_of_left_leaves(tree_node.left.clone()) + Solution::sum_of_left_leaves(tree_node.right.clone()) + sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_404() {
        assert_eq!(
            Solution::sum_of_left_leaves(tree![3,9,20,null,null,15,7]),
            24
        );
    }
}