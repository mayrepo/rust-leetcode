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
        max.unwrap()
    }
}

//The first part of the result is the max (non empty) path of the tree (None used only for empty tree: if left or right is None).
//It is stated that the path must contain at least one node. None means that there is no node 
//This prevents the case where all values are negative to return 0 as a solution.
//
//The second part of the result is the sum of the max path of the tree which has the root as a end. This path can be empty.
//
//The second parameter is used to recursively compute the max path including the root node
//The first parameter is used to remember of the max path of each child node
fn max_path_sum2(root: Rc<RefCell<TreeNode>>) -> (Option<i32>, i32) {
    let root = root.borrow();
    
    let (max1, max_root) = root.left.clone().map(max_path_sum2).unwrap_or((None,0));
    let (max2, max_root2) = root.right.clone().map(max_path_sum2).unwrap_or((None,0));
    (Some(max_opt(max_opt(root.val+max_root+max_root2, max1), max2)), 0.max(root.val+0.max(max_root).max(max_root2)))
}

//Helper function: optional maximum
fn max_opt<T: Ord>(v1: T, v2: Option<T>) -> T {
    match v2 {
        None => v1,
        Some(v2) => v1.max(v2),
    }
}

