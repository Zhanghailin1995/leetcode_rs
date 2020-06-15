// 501. Find Mode in Binary Search Tree
// Given a binary search tree (BST) with duplicates, find all the mode(s) (the most frequently occurred element) in the given BST.
//
// Assume a BST is defined as follows:
//
// The left subtree of a node contains only nodes with keys less than or equal to the node's key.
// The right subtree of a node contains only nodes with keys greater than or equal to the node's key.
// Both the left and right subtrees must also be binary search trees.
//
//
// For example:
// Given BST [1,null,2,2],
//
// 1
// \
// 2
// /
// 2
//
//
// return [2].
//
// Note: If a tree has more than one mode, you can return them in any order.
//
// Follow up: Could you do that without using any extra space? (Assume that the implicit stack space incurred due to recursion does not count).
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
use std::cmp::max;

impl Solution {
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut res = Vec::new();
        let mut max = 0;
        fn helper(node: Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, i32>, res: &mut Vec<i32>, max: &mut i32) -> i32 {
            if node.is_none() {
                return 0;
            }
            let tree_node = node.as_ref().unwrap().borrow();
            let left_sum = helper(tree_node.left.clone(), map, res, max);
            let right_sum = helper(tree_node.right.clone(), map, res, max);
            let sum = left_sum + right_sum + tree_node.val;
            let mut count = *map.get(&sum).unwrap_or(&0) + 1;
            if count > *max {
                *max = count;
                // 相对耗时
                res.clear();
            }
            if count == *max {
                res.push(sum);
            }
            //let count1 = count.to_owned();
            map.insert(sum, count);
            sum
        }
        helper(root, &mut map, &mut res, &mut max);
        res
    }

    pub fn find_frequent_tree_sum2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn walk(
            root: Option<&Rc<RefCell<TreeNode>>>,
            map: &mut HashMap<i32, i32>,
            max_value: &mut i32,
        ) -> i32 {
            if let Some(node) = root {
                let node = node.borrow();
                let left = walk(node.left.as_ref(), map, max_value);
                let right = walk(node.right.as_ref(), map, max_value);
                let sum = node.val + left + right;
                *map.entry(sum).or_default() +=1;
                *max_value = max(*max_value, *map.get(&sum).unwrap());
                sum
            } else {
                0
            }
        }

        let mut map = HashMap::new();
        let mut max_value = 0;
        let mut res = Vec::new();
        walk(root.as_ref(), &mut map, &mut max_value);
        for i in map.keys() {
            if map.get(i).unwrap() == &max_value {
                res.push(*i);
            }
        }
        res
    }

    pub fn find_frequent_tree_sum3(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut res = Vec::new();
        let mut max_val = 0;
        fn helper(node: Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, i32>, res: &mut Vec<i32>, max_val: &mut i32) -> i32 {
            if node.is_none() {
                return 0;
            }
            let tree_node = node.as_ref().unwrap().borrow();
            let left_sum = helper(tree_node.left.clone(), map, res, max_val);
            let right_sum = helper(tree_node.right.clone(), map, res, max_val);
            let sum = left_sum + right_sum + tree_node.val;
            *map.entry(sum).or_default() +=1;
            *max_val = max(*max_val, *map.get(&sum).unwrap());
            sum
        }
        helper(root, &mut map, &mut res, &mut max_val);
        for i in map.keys() {
            if map.get(i).unwrap() == &max_val {
                res.push(*i);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_508() {
        assert_eq!(Solution::find_frequent_tree_sum3(tree![5,2,-5]), vec![2])
    }

    #[test]
    fn test1() {
        let a = &mut 4;
        let b = &3;
        let c = &4;
        *a = 5;
        println!("{}", a)
    }
}