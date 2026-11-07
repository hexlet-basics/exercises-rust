use std::collections::HashMap;

pub fn frequencies(words: Vec<String>) -> HashMap<String, usize> {
    // BEGIN
    let mut result = HashMap::new();
    for word in words {
        let count = result.entry(word).or_insert(0);
        *count += 1;
    }
    result
    // END
}
