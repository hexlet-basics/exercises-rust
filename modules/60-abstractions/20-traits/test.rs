#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        use index::Named;
        let book = index::Book {
            title: String::from("Rust"),
        };
        let user = index::User {
            name: String::from("Alice"),
        };
        assert_eq!(book.label(), "Book: Rust");
        assert_eq!(user.label(), "User: Alice");
        assert_eq!(book.label_length(), 10);
        assert_eq!(user.label_length(), 11);
        assert_eq!(book.title, "Rust");
        assert_eq!(user.name, "Alice");
        assert_eq!(
            index::Book {
                title: String::new()
            }
            .label(),
            "Book: "
        );
        assert_eq!(
            index::User {
                name: String::new()
            }
            .label(),
            "User: "
        );
    }
}
