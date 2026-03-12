fn staircase(n: i32) {
    for i in 1..=n {
        let spaces = (n - i) as usize;
        let hashes = i as usize;

        println!("{}{}", " ".repeat(spaces), "#".repeat(hashes));
    }
}

