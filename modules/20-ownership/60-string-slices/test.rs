#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::first_half("abcdef"), "abc");
        assert_eq!(index::first_half("abcde"), "ab");
        assert_eq!(index::first_half(""), "");
        assert_eq!(index::first_half("x"), "");
        let text = String::from("Rust2024");
        let result = index::first_half(&text);
        assert_eq!(result, "Rust");
        assert!(std::ptr::eq(result.as_ptr(), text.as_ptr()));
        assert_eq!(text, "Rust2024");
    }
}
