#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::convert(3), 180);
        assert_eq!(index::convert(0), 0);
        assert_eq!(index::convert(1), 60);
        assert_eq!(index::convert(1000), 60000);
    }
}
