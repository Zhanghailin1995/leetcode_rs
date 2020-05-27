#[allow(dead_code)]
pub struct Solution {}
#[allow(unused_imports)]
use crate::util::tree::{to_tree, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    //https://leetcode-cn.com/problems/lowest-common-ancestor-of-a-binary-tree/solution/c-jing-dian-di-gui-si-lu-fei-chang-hao-li-jie-shi-/
    #[allow(dead_code)]
    pub fn lca_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<i32> {
        if root.is_none() {
            return None;
        }
        let tree_node = root.as_ref().unwrap().borrow();
        if tree_node.val == p || tree_node.val == q {
            return Some(tree_node.val);
        }
        let left = Solution::lca_of_binary_tree(tree_node.left.clone(), p, q);
        let right = Solution::lca_of_binary_tree(tree_node.right.clone(), p, q);
        if left.is_none() {
            return right;
        }
        if right.is_none() {
            return left;
        }
        if left.is_some() && right.is_some() {
            return Some(tree_node.val);
        }
        return None;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_236() {
        assert_eq!(Solution::lca_of_binary_tree(tree![3,5,1,6,2,0,8,null,null,7,4], 5, 4), Some(5));
        assert_eq!(Solution::lca_of_binary_tree(tree![3,5,1,6,2,0,8,null,null,7,4], 5, 1), Some(5));
    }
}