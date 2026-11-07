#[derive(Debug, PartialEq)]
pub enum Measurement {
    Missing,
    Value(i32),
}

pub fn measurement(available: bool, value: i32) -> Measurement {
    // BEGIN
    if available {
        Measurement::Value(value)
    } else {
        Measurement::Missing
    }
    // END
}
