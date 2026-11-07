#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        let mut prices = std::collections::HashMap::new();
        prices.insert(String::from("Rust"), 500);
        prices.insert(String::from("Go"), 400);
        assert_eq!(index::update_price(&mut prices, "Rust", 650), Some(500));
        assert_eq!(prices.get("Rust"), Some(&650));
        assert_eq!(prices.get("Go"), Some(&400));
        assert_eq!(index::update_price(&mut prices, "Python", 300), None);
        assert!(!prices.contains_key("Python"));
        assert_eq!(prices.len(), 2);
        assert_eq!(index::update_price(&mut prices, "Rust", 0), Some(650));
        assert_eq!(prices.get("Rust"), Some(&0));
        let mut empty = std::collections::HashMap::new();
        assert_eq!(index::update_price(&mut empty, "Missing", 10), None);
        assert!(empty.is_empty());
    }
}
