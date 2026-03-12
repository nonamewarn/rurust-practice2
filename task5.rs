fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let mut apple_count = 0;
    let mut orange_count = 0;

    for &apple in apples {
        let pos = a + apple;
        if pos >= s && pos <= t {
            apple_count += 1;
        }
    }

    for &orange in oranges {
        let pos = b + orange;
        if pos >= s && pos <= t {
            orange_count += 1;
        }
    }

    println!("{}", apple_count);
    println!("{}", orange_count);
}
