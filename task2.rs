fn diagonal_difference(arr: Vec<Vec<i32>>) -> i32 {
    let n = arr.len();
    let mut primary = 0;
    let mut secondary = 0;

    for i in 0..n {
        primary += arr[i][i];
        secondary += arr[i][n - 1 - i];
    }

    (primary - secondary).abs()
}
