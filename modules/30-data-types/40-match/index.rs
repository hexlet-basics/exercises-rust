pub enum Operation {
    Add(i32, i32),
    Multiply(i32, i32),
    Negate(i32),
}

pub fn evaluate(operation: Operation) -> i32 {
    // BEGIN
    match operation {
        Operation::Add(left, right) => left + right,
        Operation::Multiply(left, right) => left * right,
        Operation::Negate(value) => -value,
    }
    // END
}
