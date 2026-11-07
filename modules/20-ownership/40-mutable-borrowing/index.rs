pub fn add_exclamation(text: &mut String) {
    text.push('!');
}

pub fn emphasize(text: String) -> String {
    // BEGIN
    let mut text = text;
    add_exclamation(&mut text);
    add_exclamation(&mut text);
    text
    // END
}
