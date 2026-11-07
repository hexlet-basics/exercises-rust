#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::count_character(String::from("мама"), 'м'), 2);
        assert_eq!(index::count_character(String::new(), 'м'), 0);
        assert_eq!(index::count_character(String::from("Rust rust"), 'R'), 1);
        assert_eq!(index::count_character(String::from("🙂x🙂"), '🙂'), 2);
        assert_eq!(index::count_character(String::from("abc"), 'z'), 0);
    }
}
