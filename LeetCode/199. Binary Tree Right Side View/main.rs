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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        use std::collections::VecDeque;
        use std::rc::Rc;

        let mut result = vec![];
        if root.is_none() {
            return result;
        }
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let mut size = queue.len();
            while size > 0 {
                let node = queue.pop_front().unwrap().unwrap();
                let node = Rc::try_unwrap(node).unwrap().into_inner();
                if size == 1 {
                    result.push(node.val);
                }
                if let Some(left) = node.left.clone() {
                    queue.push_back(Some(left));
                }
                if let Some(right) = node.right.clone() {
                    queue.push_back(Some(right));
                }
                size -= 1;
            }
        }
        result
    }
}