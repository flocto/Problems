use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = Vec::new();
        let mut p: usize = 0;
        let mut i: usize = 0;
        use std::i32::MIN;
        let mut root = Rc::new(RefCell::new(TreeNode::new(MIN))); // dummy node

        let _r = root.clone();

        while i != inorder.len() {
            let node_r = Rc::new(RefCell::new(TreeNode::new(preorder[p])));
            stack.push(node_r.clone());
            root.borrow_mut().right = Some(node_r.clone());
            root = node_r;

            while preorder[p] != inorder[i] {
                p += 1;
                let node_l = Rc::new(RefCell::new(TreeNode::new(preorder[p])));
                stack.push(node_l.clone());
                root.borrow_mut().left = Some(node_l.clone());
                root = node_l;
            }

            while !stack.is_empty() && {
                let v = stack.last().unwrap().borrow().val;
                v == inorder[i]
            } {
                i += 1;
                root = stack.pop().unwrap();
            }
            p += 1;
        }
        let root = _r.borrow().right.clone();
        root
    }
}