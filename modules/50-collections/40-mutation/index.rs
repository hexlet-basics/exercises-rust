pub fn normalize(values: &mut [i32]) {
    // BEGIN
    for value in &mut *values {
        if *value < 0 {
            *value = 0;
        }
    }
    values.sort();
    // END
}
