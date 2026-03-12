fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {
        return if x1 == x2 { "YES".to_string() } else { "NO".to_string() };
    }

    let diff_pos = x2 - x1;
    let diff_vel = v1 - v2;

    if diff_pos % diff_vel == 0 && diff_pos / diff_vel >= 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}
