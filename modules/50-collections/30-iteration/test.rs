#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        let names = vec![
            String::from("Amy"),
            String::from("Robert"),
            String::from("Jo"),
        ];
        assert_eq!(index::count_long_names(&names, 3), 2);
        assert_eq!(index::count_long_names(&names, 6), 1);
        assert_eq!(index::count_long_names(&names, 7), 0);
        assert_eq!(names[0], "Amy");
        assert_eq!(names.len(), 3);
        assert_eq!(index::count_long_names(&[], 0), 0);
        let texts = [String::new(), String::from("Я")];
        assert_eq!(index::count_long_names(&texts, 0), 2);
        assert_eq!(index::count_long_names(&texts, 2), 1);
    }
}
