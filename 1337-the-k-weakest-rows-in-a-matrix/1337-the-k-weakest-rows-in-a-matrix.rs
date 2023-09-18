use std::cmp::Ordering;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut row_strengths: Vec<(i32, i32)> = mat.iter().enumerate().map(|(index, row)| {
            (row.iter().sum(), index as i32)
        }).collect();

        row_strengths.sort_by(|a, b| {
            match a.0.cmp(&b.0) {
                Ordering::Equal => a.1.cmp(&b.1),
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less
            }
        });

        // iter()를 사용하려면 역참조 사용해야함. iter()는 불변.
        // into_iter()는 소유권 이전.
        //row_strengths.iter().map(|(_, i)| *i).take(k as usize).collect()
        row_strengths.into_iter().map(|(_, i)| i).take(k as usize).collect()

    }
}