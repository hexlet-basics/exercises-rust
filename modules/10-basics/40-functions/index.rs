pub fn to_seconds(minutes: i32, extra_seconds: i32) -> i32 {
    // BEGIN
    let base = {
        let seconds_per_minute = 60;
        minutes * seconds_per_minute
    };
    base + extra_seconds
    // END
}
