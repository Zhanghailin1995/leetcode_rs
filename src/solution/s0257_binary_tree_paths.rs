/**
 * [257] Binary Tree Paths
 *
 * Given a binary tree, return all root-to-leaf paths.
 *
 * Note: A leaf is a node with no children.
 *
 * Example:
 *
 *
 * Input:
 *
 *    1
 *  /   \
 * 2     3
 *  \
 *   5
 *
 * Output: ["1->2->5", "1->3"]
 *
 * Explanation: All root-to-leaf paths are: 1->2->5, 1->3
 *
 */
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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut res = Vec::new();
        Solution::helper(root, "".to_owned(), &mut res);
        res
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, path: String, res: &mut Vec<String>) {
        if root.is_none() {
            return;
        }
        let tree_node = root.as_ref().unwrap().borrow();
        match (tree_node.left.clone(), tree_node.right.clone()) {
            (None, None) => res.push(format!("{}{}", path, tree_node.val)),
            _ => {
                let path = format!("{}{}->", path, tree_node.val);
                Solution::helper(tree_node.left.clone(), path.clone(), res);
                Solution::helper(tree_node.right.clone(), path.clone(), res);
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_257() {
        assert_eq!(
            Solution::binary_tree_paths(tree![1, 2, 3, null, 5]),
            vec_string!["1->2->5", "1->3"]
        );
    }
}
