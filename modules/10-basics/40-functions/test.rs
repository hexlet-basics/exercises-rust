#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::to_seconds(2, 15), 135);
        assert_eq!(index::to_seconds(0, 0), 0);
        assert_eq!(index::to_seconds(0, 59), 59);
        assert_eq!(index::to_seconds(1000, 59), 60059);
    }
}
