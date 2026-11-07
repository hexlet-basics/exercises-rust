pub mod units {
    pub fn minutes_to_seconds(minutes: i32) -> i32 {
        minutes * 60
    }
}

// BEGIN
pub fn convert(minutes: i32) -> i32 {
    units::minutes_to_seconds(minutes)
}
// END
