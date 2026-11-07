#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::sum_prices([100, 250, 50]), 400);
        assert_eq!(index::sum_prices([0, 0, 0]), 0);
        assert_eq!(index::sum_prices([1, 2, 3]), 6);
        assert_eq!(
            index::sum_prices([1_000_000, 1_000_000, 1_000_000]),
            3_000_000
        );
    }
}
