pub fn check_even(value: i32) -> Result<i32, String> {
    if value < 0 {
        Err(String::from("negative"))
    } else if value % 2 != 0 {
        Err(String::from("odd"))
    } else {
        Ok(value)
    }
}

pub fn sum_even(left: i32, right: i32) -> Result<i32, String> {
    // BEGIN
    let left = check_even(left)?;
    let right = check_even(right)?;
    Ok(left + right)
    // END
}
