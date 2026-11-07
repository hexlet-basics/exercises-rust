pub fn count_long_names(names: &[String], minimum_bytes: usize) -> usize {
    // BEGIN
    let mut count = 0;
    for name in names {
        if name.len() >= minimum_bytes {
            count += 1;
        }
    }
    count
    // END
}
