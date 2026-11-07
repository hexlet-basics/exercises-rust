#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::quotient_and_remainder(7, 2), (3, 1));
        assert_eq!(index::quotient_and_remainder(-7, 2), (-3, -1));
        assert_eq!(index::quotient_and_remainder(0, 3), (0, 0));
        assert_eq!(index::quotient_and_remainder(9, -4), (-2, 1));
    }
}
