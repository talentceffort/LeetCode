use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let root = find_root(n, &left_child, &right_child);
        
        if root == -1 {
            return false;
        }

        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        seen.insert(root);
        queue.push_back(root);

        while !queue.is_empty() {
            let node = queue.pop_back().unwrap() as usize;
            let children = vec![left_child[node], right_child[node]];

            for child in children {
                if child != -1 {
                    if seen.contains(&child) {
                        return false;
                    }
                    queue.push_back(child);
                    seen.insert(child);
                }
            }
        }

        return seen.len() == n as usize;

        fn find_root(n: i32, left_child: &Vec<i32>, right_child: &Vec<i32>) -> i32 {
            let mut children = HashSet::new();

            for node in left_child {
                children.insert(node);
            }

            for node in right_child {
                children.insert(node);
            }

            for i in 0..n {
                if !children.contains(&i) {
                    return i;
                }
            }
            -1
        }
    }
}