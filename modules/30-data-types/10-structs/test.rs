#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        let user = index::User {
            name: String::from("Alice"),
            active: true,
        };
        assert_eq!(
            index::rename_user(user, String::from("Bob")),
            index::User {
                name: String::from("Bob"),
                active: true
            }
        );
        let user = index::User {
            name: String::from("Old"),
            active: false,
        };
        assert_eq!(
            index::rename_user(user, String::new()),
            index::User {
                name: String::new(),
                active: false
            }
        );
        let user = index::User {
            name: String::new(),
            active: false,
        };
        assert_eq!(
            index::rename_user(user, String::from("New")),
            index::User {
                name: String::from("New"),
                active: false
            }
        );
    }
}
