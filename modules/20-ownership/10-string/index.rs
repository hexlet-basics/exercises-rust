pub fn excited_label(exclamations: i32) -> String {
    // BEGIN
    let mut label = String::from("Rust");
    for _ in 0..exclamations {
        label.push('!');
    }
    label
    // END
}
