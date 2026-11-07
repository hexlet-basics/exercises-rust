#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::parse_quantity(" 3 "), Ok(3));
        assert_eq!(index::parse_quantity("1"), Ok(1));
        assert_eq!(index::parse_quantity("1000"), Ok(1000));
        for text in ["0", "-1", "1001", "2147483647"] {
            assert_eq!(
                index::parse_quantity(text),
                Err(String::from("out of range"))
            );
        }
        for text in ["", " ", "abc", "1.5", "2147483648", "1 2"] {
            assert_eq!(
                index::parse_quantity(text),
                Err(String::from("invalid number"))
            );
        }
        assert_eq!(
            index::parse_quantity(
                "
12	"
            ),
            Ok(12)
        );
    }
}
