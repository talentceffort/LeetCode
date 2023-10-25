impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        // (k - 1).count_ones() as i32 & 1
        // 0, 
        // 10,
        // 0110 -> 10011001
        // 0110 1001
        // 
       if n == 1 {
            return 0;
        }
        
        let total_nodes = 2i32.pow((n - 1) as u32);
        
        if k <= (total_nodes / 2) {
            Solution::kth_grammar(n - 1, k)
        } else {
            1 - Solution::kth_grammar(n - 1, k - (total_nodes / 2))
        }
    }
}