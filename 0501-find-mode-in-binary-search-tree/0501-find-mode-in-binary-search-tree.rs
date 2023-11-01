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
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let None = root { return vec![] }
        
        let root = root.unwrap();

        let mut map: HashMap<i32, u32> = HashMap::new();
        
        Self::dfs(root, &mut map);

        let max_mode = *map.values().max().unwrap();
        
        map.into_iter()
            .filter_map(|(n, mode)| (mode == max_mode).then(|| n))
            .collect()
    }

    fn dfs(node: Rc<RefCell<TreeNode>>, map: &mut HashMap<i32, u32>) {
        map.entry(node.borrow().val)
            .and_modify(|v| *v += 1)
            .or_insert(1);
        if let Some(l) = node.borrow().left.clone()  { Self::dfs(l, map) }
        if let Some(r) = node.borrow().right.clone() { Self::dfs(r, map) }
    }
}