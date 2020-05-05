/**
 * [113] Path Sum II
 *
 * Given a binary tree and a sum, find all root-to-leaf paths where each path's sum equals the given sum.
 *
 * Note: A leaf is a node with no children.
 *
 * Example:
 *
 * Given the below binary tree and sum = 22,
 *
 *
 *       5
 *      / \
 *     4   8
 *    /   / \
 *   11  13  4
 *  /  \    / \
 * 7    2  5   1
 *
 *
 * Return:
 *
 *
 * [
 *    [5,4,11,2],
 *    [5,8,4,5]
 * ]
 *
 *
 */
pub struct Solution {}

use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/path-sum-ii/
// discuss: https://leetcode.com/problems/path-sum-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut tmp = Vec::new();
        fn backtrack(node: Option<Rc<RefCell<TreeNode>>>, sum: i32, res: &mut Vec<Vec<i32>>, tmp: &mut Vec<i32>) {
            if node.is_none() {
                return;
            }
            let tree_node = node.as_ref().unwrap().borrow();
            tmp.push(tree_node.val);
            match (tree_node.left.clone(), tree_node.right.clone()) {
                (None, None) => {
                    if sum == tree_node.val {
                        res.push(tmp.to_vec());
                    }
                }
                (l, r) => {
                    backtrack(l, sum - tree_node.val, res, tmp);
                    backtrack(r, sum - tree_node.val, res, tmp);
                }
            }
            tmp.pop();
        }
        backtrack(root, sum, &mut res, &mut tmp);
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_113() {
        assert_eq!(
            Solution::path_sum(tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1], 22),
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
        )
    }
}
