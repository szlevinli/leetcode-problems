pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::cmp::max;
        let mut result: Vec<Vec<i32>> = Vec::new();

        // let mut _intervals = intervals.clone();
        let mut _intervals = Vec::with_capacity(intervals.len());
        _intervals.extend(&intervals);

        _intervals.sort_by(|&a, &b| b[0].cmp(&a[0]));

        while let Some(current) = _intervals.pop() {
            if let Some(prev) = result.last_mut() {
                if prev[1] >= current[0] {
                    prev[1] = max(prev[1], current[1]);
                } else {
                    result.push(current.clone());
                }
            } else {
                result.push(current.clone());
            }
        }

        result
    }
}
