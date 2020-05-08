/**
 * [199] Binary Tree Right Side View
 *
 * Given a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.
 *
 * Example:
 *
 *
 * Input: [1,2,3,null,5,null,4]
 * Output: [1, 3, 4]
 * Explanation:
 *
 *    1            <---
 *  /   \
 * 2     3         <---
 *  \     \
 *   5     4       <---
 *
 */
pub struct Solution {}

use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/binary-tree-right-side-view/
// discuss: https://leetcode.com/problems/binary-tree-right-side-view/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        if root.is_none() {
            return res;
        }
        let mut linked_list = LinkedList::new();
        linked_list.push_back(root.clone());

        while !linked_list.is_empty() {
            let size = linked_list.len();
            for i in 0..size {
                let node = linked_list.pop_front().unwrap();
                let tree_node = node.as_ref().unwrap().borrow();
                if i == size - 1 {
                    res.push(tree_node.val)
                }
                if tree_node.left.is_some() {
                    linked_list.push_back(tree_node.left.clone())
                }
                if tree_node.right.is_some() {
                    linked_list.push_back(tree_node.right.clone())
                }
            }
        }
        res
    }

    pub fn right_side_view2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>, mut depth: i32) {
            if node.is_none() {
                return;
            }
            let tree_node = node.as_ref().unwrap().borrow();
            if depth == vec.len() as i32 {
                vec.push(tree_node.val)
            }
            depth += 1;
            dfs(tree_node.right.clone(), vec, depth);
            dfs(tree_node.left.clone(), vec, depth);
        }
        dfs(root, &mut res, 0);
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_199() {
        assert_eq!(
            Solution::right_side_view(tree![1, 2, 3, null, 5, null, 4]),
            vec![1, 3, 4]
        );
    }
}
