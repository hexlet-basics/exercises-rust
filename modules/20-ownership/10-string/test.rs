#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::excited_label(0), "Rust");
        assert_eq!(index::excited_label(1), "Rust!");
        assert_eq!(index::excited_label(3), "Rust!!!");
        assert_eq!(index::excited_label(20), format!("Rust{}", "!".repeat(20)));
    }
}
