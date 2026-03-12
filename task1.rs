fn birthday_cake_candles(candles: Vec<i32>) -> i32 {
    let max_height = candles.iter().max().unwrap();
    candles.iter().filter(|&&h| h == *max_height).count() as i32
}
