/**
 * [106] Construct Binary Tree from Inorder and Postorder Traversal
 *
 * Given inorder and postorder traversal of a tree, construct the binary tree.
 *
 * Note:<br />
 * You may assume that duplicates do not exist in the tree.
 *
 * For example, given
 *
 *
 * inorder = [9,3,15,20,7]
 * postorder = [9,15,7,20,3]
 *
 * Return the following binary tree:
 *
 *
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
 *
 *
 */
#[allow(dead_code)]
pub struct Solution {}
#[allow(unused_imports)]
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
// discuss: https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    #[allow(dead_code)]
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::build_tree_helper(&postorder[..], &inorder[..])
    }

    fn build_tree_helper(postorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.is_empty() {
            return None;
        }
        let root_idx = inorder
            .iter()
            .position(|v| v == postorder.last().unwrap())
            .unwrap();
        Some(Rc::new(RefCell::new(TreeNode {
            val: *postorder.last().unwrap(),
            right: Solution::build_tree_helper(
                &postorder[root_idx..postorder.len() - 1],
                &inorder[root_idx + 1..],
            ),
            left: Solution::build_tree_helper(&postorder[0..root_idx], &inorder[0..root_idx]),
        })))
    }
}