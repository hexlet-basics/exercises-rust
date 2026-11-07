// BEGIN
pub fn access_level(age: i32, has_ticket: bool, is_blocked: bool) -> i32 {
    if is_blocked {
        0
    } else if age >= 18 && has_ticket {
        2
    } else {
        1
    }
}
// END
