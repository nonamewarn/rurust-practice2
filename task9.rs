fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut freq = [0; 6];

    for &bird in arr {
        freq[bird as usize] += 1;
    }

    let mut max_count = 0;
    let mut result = 0;

    for i in 1..6 {
        if freq[i] > max_count {
            max_count = freq[i];
            result = i as i32;
        }
    }

    result
}
