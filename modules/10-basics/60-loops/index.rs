// BEGIN
pub fn next_multiple_of_three(start: i32) -> i32 {
    let mut number = start;
    loop {
        number += 1;
        if number % 3 == 0 {
            break number;
        }
    }
}
// END
