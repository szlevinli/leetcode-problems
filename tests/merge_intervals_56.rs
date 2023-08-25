use leetcode_problems::medium::merge_intervals_56::Solution;

#[test]
fn test_empty_intervals() {
    let result = Solution::merge(Vec::new());
    let expect: Vec<Vec<i32>> = Vec::new();
    assert_eq!(result, expect);
}

#[test]
fn test_can_merge() {
    let result = Solution::merge(vec![vec![1, 4], vec![4, 5]]);
    let expect: Vec<Vec<i32>> = vec![vec![1, 5]];
    assert_eq!(result, expect);
}

#[test]
fn test_not_merge() {
    let result = Solution::merge(vec![vec![14, 16], vec![9, 10]]);
    let expect: Vec<Vec<i32>> = vec![vec![9, 10], vec![14, 16]];
    assert_eq!(result, expect);
}

#[test]
fn test_all_merge() {
    let result = Solution::merge(vec![vec![2, 5], vec![1, 16], vec![9, 10]]);
    let expect: Vec<Vec<i32>> = vec![vec![1, 16]];
    assert_eq!(result, expect);
}
