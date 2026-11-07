pub fn consume(text: String) {
    println!("{}", text);
}

pub fn keep_original(text: String) -> String {
    // BEGIN
    consume(text.clone());
    text
    // END
}
