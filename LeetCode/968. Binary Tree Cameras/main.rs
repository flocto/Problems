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
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let ans = Solution::dfs(root, &mut res);
        if ans == 0 { //if the root is not covered
            return res + 1;
        }
        res
    }
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
        if root.is_none() { //null root is same as covered
            return 2;
        }
        let mut left = Solution::dfs(root.as_ref().unwrap().borrow().left.clone(), res);
        let mut right = Solution::dfs(root.as_ref().unwrap().borrow().right.clone(), res);
        if left == 0 || right == 0 { //if the left or right is not covered, then put camera on root
            *res += 1;
            return 1;
        }
        if left == 1 || right == 1 { //if left or right has camera, then ignore
            return 2;
        }
        0 //root is a leaf
    }
}