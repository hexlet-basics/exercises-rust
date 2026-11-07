pub fn safe_divide(numerator: i32, denominator: i32) -> Result<i32, String> {
    // BEGIN
    if denominator == 0 {
        Err(String::from("division by zero"))
    } else {
        Ok(numerator / denominator)
    }
    // END
}
