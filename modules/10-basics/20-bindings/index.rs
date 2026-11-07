pub fn show_values() {
    // BEGIN
    const STEP: i32 = 5;
    let original = 8;
    let mut value = original;
    value += STEP;
    let value = value * 2;
    println!("{}", original);
    println!("{}", value);
    // END
}
