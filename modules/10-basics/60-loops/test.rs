#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::next_multiple_of_three(0), 3);
        assert_eq!(index::next_multiple_of_three(3), 6);
        assert_eq!(index::next_multiple_of_three(4), 6);
        assert_eq!(index::next_multiple_of_three(5), 6);
        assert_eq!(index::next_multiple_of_three(1_000_000), 1_000_002);
    }
}
