//437. Path Sum III
// You are given a binary tree in which each node contains an integer value.
//
// Find the number of paths that sum to a given value.
//
// The path does not need to start or end at the root or a leaf, but it must go downwards (traveling only from parent nodes to child nodes).
//
// The tree has no more than 1,000 nodes and the values are in the range -1,000,000 to 1,000,000.
//
// Example:
//
// root = [10,5,-3,3,2,null,11,3,-2,null,1], sum = 8
//
//       10
//      /  \
//     5   -3
//    / \    \
//   3   2   11
//  / \   \
// 3  -2   1
//
// Return 3. The paths that sum to 8 are:
//
// 1.  5 -> 3
// 2.  5 -> 2 -> 1
// 3. -3 -> 11
#[allow(dead_code)]
pub struct Solution {}
#[allow(unused_imports)]
use crate::util::tree::{to_tree, TreeNode};
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
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut prefix_sum_count = HashMap::new();
        prefix_sum_count.insert(0, 1);
        Solution::dfs(root, &mut prefix_sum_count, sum, 0)
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, prefix_sum_count: &mut HashMap<i32, i32>, target: i32, curr_sum: i32) -> i32 {
        if node.is_none() {
            return 0;
        }
        let mut res = 0;
        let tree_node = node.as_ref().unwrap().borrow();
        let curr_sum = curr_sum + tree_node.val;
        res += *prefix_sum_count.get(&(curr_sum - target)).unwrap_or(&0);
        prefix_sum_count.insert(curr_sum, *prefix_sum_count.get(&(curr_sum)).unwrap_or(&0) + 1);
        res += Solution::dfs(tree_node.left.clone(), prefix_sum_count, target, curr_sum);
        res += Solution::dfs(tree_node.right.clone(), prefix_sum_count, target, curr_sum);
        prefix_sum_count.insert(curr_sum, prefix_sum_count.get(&curr_sum).unwrap() - 1);
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_437() {
        assert_eq!(
            Solution::path_sum(tree![1,-2,-3], -1),
            1
        );
    }
}