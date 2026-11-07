pub fn numbers_up_to(limit: i32) -> Vec<i32> {
    // BEGIN
    let mut values = Vec::new();
    for number in 1..=limit {
        values.push(number);
    }
    values
    // END
}
