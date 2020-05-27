
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
#![allow(unused_imports)]
use std::rc::Rc;
use std::cell::RefCell;
#[allow(dead_code)]
pub struct Solution {}

use crate::util::tree::{to_tree, TreeNode};
use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn build_tree2(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }

        let mut idx_map = HashMap::new();
        for (idx, num) in inorder.iter().enumerate() {
            idx_map.insert(*num, idx);
        }

        //println!("{:?}", idx_map);

        Solution::helper(&preorder, &idx_map, 0, 0, inorder.len() as i32)
    }

    fn helper(preorder: &Vec<i32>, idx_map: &HashMap<i32, usize>,  pre_root_index: usize, left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if left == right {
            return None;
        }
        let root_val = preorder[pre_root_index];
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(root_val))));

        let index = idx_map[&root_val];

        root.as_mut().unwrap().borrow_mut().left = Self::helper(preorder, idx_map, pre_root_index + 1, left, index as i32);
        root.as_mut().unwrap().borrow_mut().right = Self::helper(preorder, idx_map, pre_root_index + 1 + index - left as usize, index as i32 + 1, right);

        root
    }

    #[allow(dead_code)]
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::build_tree_helper(&preorder[..], &inorder[..])
    }

    fn build_tree_helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let root_idx = inorder.iter().position(|&v| v == preorder[0]).unwrap();
        Some(Rc::new(RefCell::new(TreeNode {
            val: preorder[0],
            left: Solution::build_tree_helper(&preorder[1..root_idx + 1], &inorder[0..root_idx]),
            right: Solution::build_tree_helper(&preorder[root_idx + 1..], &inorder[root_idx + 1..]),
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_105() {
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            tree![3, 9, 20, null, null, 15, 7]
        );
        assert_eq!(
            Solution::build_tree(vec![3, 20, 7], vec![3, 20, 7]),
            tree![3, null, 20, null, 7]
        );
        assert_eq!(Solution::build_tree(vec![], vec![]), tree![]);
    }
}
