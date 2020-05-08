/**
 * [222] Count Complete Tree Nodes
 *
 * Given a complete binary tree, count the number of nodes.
 *
 * Note:
 *
 * <u>Definition of a complete binary tree from <a href="http://en.wikipedia.org/wiki/Binary_tree#Types_of_binary_trees" target="_blank">Wikipedia</a>:</u><br />
 * In a complete binary tree every level, except possibly the last, is completely filled, and all nodes in the last level are as far left as possible. It can have between 1 and 2^h nodes inclusive at the last level h.
 *
 * Example:
 *
 *
 * Input:
 *     1
 *    / \
 *   2   3
 *  / \  /
 * 4  5 6
 *
 * Output: 6
 *
 */
pub struct Solution {}

use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/count-complete-tree-nodes/
// discuss: https://leetcode.com/problems/count-complete-tree-nodes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let tree_node = root.as_ref().unwrap().borrow();
        let left = Solution::count_level(tree_node.left.clone());
        let right = Solution::count_level(tree_node.right.clone());
        return if left == right {
            Solution::count_nodes(tree_node.right.clone()) + (1 << left)
        } else {
            Solution::count_nodes(tree_node.left.clone()) + (1 << right)
        };
    }

    fn count_level(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut level: i32 = 0;
        let mut cur = node;
        while let Some(tree_node) = cur {
            level += 1;
            cur = tree_node.as_ref().borrow().left.clone();
        }
        level
    }

    pub fn count_nodes2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                1 + Solution::count_nodes2(node.as_ref().borrow().right.clone())
                    + Solution::count_nodes2(node.as_ref().borrow().left.clone())
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_222() {
        assert_eq!(Solution::count_nodes(tree![1, 1, 1, 1, 1, 1, 1]), 7);
        assert_eq!(Solution::count_nodes(tree![]), 0);
        assert_eq!(Solution::count_nodes(tree![1, 1]), 2);
        assert_eq!(Solution::count_nodes(tree![1]), 1);
        assert_eq!(Solution::count_nodes(tree![1, 1, 1]), 3);
    }
}
