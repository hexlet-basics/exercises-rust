#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::evaluate(index::Operation::Add(3, -8)), -5);
        assert_eq!(index::evaluate(index::Operation::Add(0, 0)), 0);
        assert_eq!(index::evaluate(index::Operation::Multiply(7, -2)), -14);
        assert_eq!(index::evaluate(index::Operation::Multiply(0, 5)), 0);
        assert_eq!(index::evaluate(index::Operation::Negate(8)), -8);
        assert_eq!(index::evaluate(index::Operation::Negate(-8)), 8);
        assert_eq!(index::evaluate(index::Operation::Negate(0)), 0);
    }
}
