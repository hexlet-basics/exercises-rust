// BEGIN
pub fn shorter<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() <= right.len() {
        left
    } else {
        right
    }
}
// END
