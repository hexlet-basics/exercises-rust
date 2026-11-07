use std::ops::Range;

pub fn take_two(numbers: &mut Range<i32>) -> (Option<i32>, Option<i32>) {
    // BEGIN
    (numbers.next(), numbers.next())
    // END
}
