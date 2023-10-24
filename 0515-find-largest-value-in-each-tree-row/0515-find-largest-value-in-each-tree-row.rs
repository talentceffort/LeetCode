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
use std::collections::VecDeque;
use std::cmp::max;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        
        let mut result: Vec<i32> = vec![];
        let mut queue = VecDeque::from([root]);

        while !queue.is_empty() {
            let k = queue.len();
            let mut current_max = i32::MIN;

            for i in 0..k {
                match queue.pop_front().unwrap() {
                    Some(r) => {
                        current_max = max(current_max, r.borrow().val);
                        if !r.borrow().left.is_none() {
                            queue.push_back(r.borrow().left.clone());
                        }
                        if !r.borrow().right.is_none() {
                            queue.push_back(r.borrow().right.clone());
                        }
                    }
                    None => return result
                }
            }
            result.push(current_max);
        }

        result
    }
}