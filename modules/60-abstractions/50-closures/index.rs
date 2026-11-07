pub fn apply_twice(value: i32, operation: impl Fn(i32) -> i32) -> i32 {
    operation(operation(value))
}

pub fn add_twice(value: i32, delta: i32) -> i32 {
    // BEGIN
    apply_twice(value, |number| number + delta)
    // END
}
