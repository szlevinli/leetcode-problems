use leetcode_problems::easy::summary_ranges_228::Solution;

#[test]
fn it_empty() {
    let input = Vec::new();
    let expected_output: Vec<String> = Vec::new();

    assert_eq!(Solution::summary_ranges(input), expected_output);
}

#[test]
fn it_example_1() {
    let input = vec![0, 1, 2, 4, 5, 7];
    let expected_output = vec!["0->2", "4->5", "7"];

    assert_eq!(Solution::summary_ranges(input), expected_output);
}

#[test]
fn it_example_2() {
    let input = vec![0, 2, 3, 4, 6, 8, 9];
    let expected_output = vec!["0", "2->4", "6", "8->9"];

    assert_eq!(Solution::summary_ranges(input), expected_output);
}
