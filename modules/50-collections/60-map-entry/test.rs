#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        let words = vec![
            String::from("rust"),
            String::from("go"),
            String::from("rust"),
        ];
        let result = index::frequencies(words);
        assert_eq!(result.len(), 2);
        assert_eq!(result.get("rust"), Some(&2));
        assert_eq!(result.get("go"), Some(&1));
        assert!(index::frequencies(Vec::new()).is_empty());
        let result = index::frequencies(vec![
            String::new(),
            String::from("Rust"),
            String::from("rust"),
            String::new(),
        ]);
        assert_eq!(result.get(""), Some(&2));
        assert_eq!(result.get("Rust"), Some(&1));
        assert_eq!(result.get("rust"), Some(&1));
        assert_eq!(result.len(), 3);
    }
}
