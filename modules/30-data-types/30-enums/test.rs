#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::measurement(true, 0), index::Measurement::Value(0));
        assert_eq!(index::measurement(true, -8), index::Measurement::Value(-8));
        assert_eq!(index::measurement(true, 42), index::Measurement::Value(42));
        assert_eq!(index::measurement(false, 42), index::Measurement::Missing);
        assert_eq!(index::measurement(false, 0), index::Measurement::Missing);
    }
}
