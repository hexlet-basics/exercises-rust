#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::safe_divide(7, 2), Ok(3));
        assert_eq!(index::safe_divide(-7, 2), Ok(-3));
        assert_eq!(index::safe_divide(0, 3), Ok(0));
        assert_eq!(index::safe_divide(9, -3), Ok(-3));
        assert_eq!(
            index::safe_divide(7, 0),
            Err(String::from("division by zero"))
        );
        assert_eq!(
            index::safe_divide(0, 0),
            Err(String::from("division by zero"))
        );
    }
}
