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
use std::cmp::max;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (max, _) = max_path_sum2(root.unwrap());
        max
    }
}

fn max_path_sum2(root: Rc<RefCell<TreeNode>>) -> (i32, i32) {
    let root = root.borrow();
    match (root.left.clone(), root.right.clone()) {
        (None, None) => (root.val, 0.max(root.val)),
        (Some(left), None) => {
            let (max1, max_root) = max_path_sum2(left);
            (max(max1, max_root+root.val), 0.max(root.val+0.max(max_root)))
        }
        (None, Some(right)) => {
            let (max1, max_root) = max_path_sum2(right);
            (max1.max(root.val+max_root), 0.max(root.val+0.max(max_root)))
        }
        (Some(left), Some(right)) => {
            let (max1, max_root) = max_path_sum2(left);
            let (max2, max_root2) = max_path_sum2(right);
            (max1.max(max2).max(root.val+max_root+max_root2), 0.max(root.val+0.max(max_root).max(max_root2)))
        }
    }
}
