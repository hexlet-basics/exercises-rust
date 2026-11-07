pub fn positive_squares(values: &[i32]) -> Vec<i32> {
    // BEGIN
    values
        .iter()
        .filter(|value| **value > 0)
        .map(|value| *value * *value)
        .collect()
    // END
}
