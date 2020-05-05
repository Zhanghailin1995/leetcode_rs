/**
 * [114] Flatten Binary Tree to Linked List
 *
 * Given a binary tree, flatten it to a linked list in-place.
 *
 * For example, given the following tree:
 *
 *
 *     1
 *    / \
 *   2   5
 *  / \   \
 * 3   4   6
 *
 *
 * The flattened tree should look like:
 *
 *
 * 1
 *  \
 *   2
 *    \
 *     3
 *      \
 *       4
 *        \
 *         5
 *          \
 *           6
 *
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/flatten-binary-tree-to-linked-list/
// discuss: https://leetcode.com/problems/flatten-binary-tree-to-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        // 把左树插到右树上
        match root {
            Some(node) => {
                let mut node = node.borrow_mut();
                Solution::flatten(&mut node.left);
                Solution::flatten(&mut node.right);
                // 叶子节点 or 无左子树
                if node.left.is_none() { return; }

                let mut right = node.right.take();
                node.right = node.left.take(); // 左子树放到右子树上

                // 遍历到右子树末尾，把剩余右子树接上
                // 从RUST 角度来解释，首先要拿到右节点引用
                // node 的类型是&mut Rc<RefCell<TreeNode>>
                // Rc::clone,但是克隆谁呢 node.right的类型是Option<Rc<RefCell<TreeNode>>>
                // 克隆右节点的引用，算了，编不下去了
                // 是真尼玛绕，简直就是引用地狱
                let mut p = Rc::clone(node.right.as_ref().unwrap());
                while p.borrow().right.is_some() {
                    let next_p = Rc::clone(p.borrow().right.as_ref().unwrap());
                    p = next_p;
                }
                p.borrow_mut().right = right.take();
            },
            None => {}
        }
    }

}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_114() {
        let mut tree = tree![1, 2, 5, 3, 4, null, 6];
        Solution::flatten(&mut tree);
        assert_eq!(tree, tree![1, null, 2, null, 3, null, 4, null, 5, null, 6]);

        let mut tree = tree![1, 2, null, 3];
        Solution::flatten(&mut tree);
        assert_eq!(tree, tree![1, null, 2, null, 3]);

        let mut tree = tree![1, null, 2, 3];
        Solution::flatten(&mut tree);
        assert_eq!(tree, tree![1, null, 2, null, 3]);
    }
}
