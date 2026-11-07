pub fn first_positive(values: [i32; 5]) -> Option<i32> {
    // BEGIN
    for value in values {
        if value > 0 {
            return Some(value);
        }
    }
    None
    // END
}
