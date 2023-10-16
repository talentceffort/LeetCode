impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        // 0: [1]
        // 1: [1, 1]
        // 2: [1, 2, 1]
        // 3: [1, 3, 3, 1]
        // 4: [1, 4, 6, 4, 1]
        // 5: [1, 5, 10, 10, 5, 1]
        // 6: [1, 6, 15, 20, 15, 6, 1]
        // 7: [1, 7, 21, 35, 35, 21, 7, 1]

        // n - 1: 
        // n: [1, n-1[0] + n-1[1], n-1[1] + n-1[2], n, 1]

        let row_index = row_index as usize;
        
        let mut a = vec![1; row_index + 1];
        let mut b = vec![1; row_index + 1];

        for i in 1..row_index {
            for j in 1..=i {
                b[j] = a[j - 1] + a[j];
            }
            std::mem::swap(&mut a, &mut b);
        }
        a
    }
}