pub fn sum_without_first(values: &[i32]) -> i32 {
    // BEGIN
    if values.is_empty() {
        return 0;
    }
    let mut total = 0;
    for value in &values[1..] {
        total += *value;
    }
    total
    // END
}
