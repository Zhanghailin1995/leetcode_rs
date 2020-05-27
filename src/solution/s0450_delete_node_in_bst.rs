// 450. Delete Node in a BST
// Given a root node reference of a BST and a key, delete the node with the given key in the BST. Return the root node reference (possibly updated) of the BST.
//
// Basically, the deletion can be divided into two stages:
//
// Search for a node to remove.
// If the node is found, delete the node.
// Note: Time complexity should be O(height of tree).
//
// Example:
//
// root = [5,3,6,2,4,null,7]
// key = 3
//
// 5
// / \
// 3   6
// / \   \
// 2   4   7
//
// Given key to delete is 3. So we find the node with value 3 and delete it.
//
// One valid answer is [5,4,6,2,null,null,7], shown in the following BST.
//
// 5
// / \
// 4   6
// /     \
// 2       7
//
// Another valid answer is [5,2,6,null,4,null,7].
//
// 5
// / \
// 2   6
// \   \
// 4   7
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

impl Solution {
    #[allow(dead_code)]
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let mut tree_node = root.as_ref().unwrap().borrow_mut();
        if key < tree_node.val {
            tree_node.left = Solution::delete_node(tree_node.left.clone(), key);
        } else if key > tree_node.val {
            tree_node.right = Solution::delete_node(tree_node.right.clone(), key);
        } else {
            return if tree_node.left.is_none() {
                tree_node.right.clone()
            } else if tree_node.right.is_none() {
                tree_node.left.clone()
            } else {
                let mut tmp = tree_node.right.clone();
                while tmp.clone().unwrap().borrow().left.is_some() {
                    tmp = tmp.clone().unwrap().borrow().left.clone();
                    //println!("{}",tmp.clone().unwrap().borrow().val)
                }

                tmp.unwrap().borrow_mut().left = tree_node.left.clone();
                // 这里用clone 不行,需要用take,不明白为什么,性能不是很好,用了4ms,java 才0ms
                tree_node.right.take()
            };
        }
        root.clone()
    }

    #[allow(dead_code)]
    pub fn delete_node2(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(root: Option<&Rc<RefCell<TreeNode>>>,key: i32,l: Option<Rc<RefCell<TreeNode>>>,) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = root {
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();
                return if key < node.borrow().val {
                    node.as_ref().borrow_mut().left = helper(left.as_ref(), key, l);
                    Some(node.clone())
                } else if key > node.borrow().val {
                    node.as_ref().borrow_mut().right = helper(right.as_ref(), key, l);
                    Some(node.clone())
                } else {
                    helper(right.as_ref(), key, node.borrow().left.clone())
                }
            } else {
                l
            }
        }
        helper(root.as_ref(), key, None)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_450() {
        /*assert_eq!(
             Solution::delete_node(tree![5,3,6,2,4,null,7], 3),
             tree![5,4,6,2,null,null,7]
         );*/

        assert_eq!(
            Solution::delete_node(tree![2,0,33,null,1,25,40,null,null,11,31,34,45,10,18,29,32,null,36,43,46,4,null,12,24,26,30,null,null,35,39,42,44,null,48,3,9,null,14,22,null,null,27,null,null,null,null,38,null,41,null,null,null,47,49,null,null,5,null,13,15,21,23,null,28,37,null,null,null,null,null,null,null,null,8,null,null,null,17,19,null,null,null,null,null,null,null,7,null,16,null,null,20,6], 33),
            tree![2,0,34,null,1,25,40,null,null,11,31,35,45,10,18,29,32,null,36,43,46,4,null,12,24,26,30,null,null,null,39,42,44,null,48,3,9,null,14,22,null,null,27,null,null,38,null,41,null,null,null,47,49,null,null,5,null,13,15,21,23,null,28,37,null,null,null,null,null,null,null,null,8,null,null,null,17,19,null,null,null,null,null,null,null,7,null,16,null,null,20,6]
        )
    }
}