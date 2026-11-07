#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::add_twice(5, 3), 11);
        assert_eq!(index::add_twice(10, -2), 6);
        assert_eq!(index::add_twice(8, 0), 8);
        assert_eq!(index::add_twice(-5, 4), 3);
        assert_eq!(index::add_twice(1_000_000, 1_000_000), 3_000_000);
    }
}
