fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    let mut count = 0;

    let max_a = *a.iter().max().unwrap();
    let min_b = *b.iter().min().unwrap();

    for x in max_a..=min_b {
        let valid_a = a.iter().all(|&ai| x % ai == 0);
        let valid_b = b.iter().all(|&bi| bi % x == 0);

        if valid_a && valid_b {
            count += 1;
        }
    }

    count
}
