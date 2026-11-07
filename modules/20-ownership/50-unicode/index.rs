pub fn count_character(text: String, target: char) -> usize {
    // BEGIN
    let mut count = 0;
    for character in text.chars() {
        if character == target {
            count += 1;
        }
    }
    count
    // END
}
