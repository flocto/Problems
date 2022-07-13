use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        if root.is_none() {
            return res;
        }
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());
        while !queue.is_empty() {
            let mut level = Vec::new();
            let mut size = queue.len();
            while size > 0 {
                let node = queue.pop_front().unwrap();
                level.push(node.borrow().val);
                if node.borrow().left.is_some() {
                    queue.push_back(node.borrow().left.as_ref().unwrap().clone());
                }
                if node.borrow().right.is_some() {
                    queue.push_back(node.borrow().right.as_ref().unwrap().clone());
                }
                size -= 1;
            }
            res.push(level);
        }
        res
    }
}