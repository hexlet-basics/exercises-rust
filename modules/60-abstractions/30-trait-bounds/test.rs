#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::max_ref::<i32>(&[]), None);
        let values = [2, 9, 9, 4];
        let found = index::max_ref(&values).expect("a maximum should exist");
        assert_eq!(*found, 9);
        assert!(std::ptr::eq(found, &values[1]));
        assert_eq!(index::max_ref(&[-5, -2, -9]), Some(&-2));
        let words = [
            String::from("apple"),
            String::from("pear"),
            String::from("pear"),
        ];
        let found = index::max_ref(&words).expect("a word should exist");
        assert_eq!(found, "pear");
        assert!(std::ptr::eq(found, &words[1]));
        assert_eq!(words[0], "apple");
    }
}
