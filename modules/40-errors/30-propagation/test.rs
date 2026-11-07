#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::sum_even(2, 4), Ok(6));
        assert_eq!(index::sum_even(0, 0), Ok(0));
        assert_eq!(index::sum_even(1_000_000, 1_000_000), Ok(2_000_000));
        assert_eq!(index::sum_even(-2, 3), Err(String::from("negative")));
        assert_eq!(index::sum_even(3, -2), Err(String::from("odd")));
        assert_eq!(index::sum_even(2, 3), Err(String::from("odd")));
        assert_eq!(index::sum_even(2, -2), Err(String::from("negative")));
    }
}
