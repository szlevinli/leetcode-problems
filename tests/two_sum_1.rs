use leetcode_problems::easy::two_sum_1::Solution;

#[test]
fn it_example_1() {
    let input = vec![2, 7, 11, 15];
    let expected_output = vec![0, 1];

    assert_eq!(Solution::two_sum(input, 9), expected_output);
}

#[test]
fn it_example_2() {
    let input = vec![3, 2, 4];
    let expected_output = vec![1, 2];

    assert_eq!(Solution::two_sum(input, 6), expected_output);
}

#[test]
fn it_example_3() {
    let input = vec![3, 3];
    let expected_output = vec![0, 1];

    assert_eq!(Solution::two_sum(input, 6), expected_output);
}

#[test]
fn it_have_tow_result_should_return_first_result() {
    let input = vec![2, 1, 9, 8, 7, 15];
    let expected_output = vec![1, 3];

    assert_eq!(Solution::two_sum(input, 9), expected_output);
}
