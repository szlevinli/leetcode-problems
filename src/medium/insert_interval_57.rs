pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut new_interval = new_interval;
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(intervals.len() + 1);

        for (i, v) in intervals.iter().enumerate() {
            match v {
                k if &k[1] < &new_interval[0] => ans.push(k.clone()),
                k if &new_interval[1] < &k[0] => {
                    ans.push(new_interval.clone());
                    ans.extend(intervals[i..].iter().cloned());
                    return ans;
                }
                k => {
                    new_interval[0] = std::cmp::min(new_interval[0], k[0]);
                    new_interval[1] = std::cmp::max(new_interval[1], k[1]);
                }
            }
        }

        ans.push(new_interval);

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_intervals_is_empty() {
        let intervals = vec![];
        let new_interval = [1, 2];
        let expected = [[1, 2]];

        assert_eq!(Solution::insert(intervals, new_interval.to_vec()), expected);
    }

    #[test]
    fn it_example_1() {
        let intervals = [[1, 3], [6, 9]].map(|item| item.to_vec()).to_vec();
        let new_interval = [2, 5].to_vec();
        let expected = [[1, 5], [6, 9]].map(|item| item.to_vec()).to_vec();

        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }

    #[test]
    fn it_example_2() {
        let intervals = [[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]]
            .map(|item| item.to_vec())
            .to_vec();
        let new_interval = [4, 8].to_vec();
        let expected = [[1, 2], [3, 10], [12, 16]]
            .map(|item| item.to_vec())
            .to_vec();

        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }

    #[test]
    fn it_example_3() {
        let intervals = [[1, 5]].map(|item| item.to_vec()).to_vec();
        let new_interval = [2, 3].to_vec();
        let expected = [[1, 5]].map(|item| item.to_vec()).to_vec();

        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }
}
