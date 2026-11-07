#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::take_last(vec![1, 2, 3]), Some(3));
        assert_eq!(index::take_last::<i32>(Vec::new()), None);
        assert_eq!(
            index::take_last(vec![String::from("first"), String::from("last")]),
            Some(String::from("last"))
        );
        struct Record {
            id: i32,
        }
        let result = index::take_last(vec![Record { id: 7 }, Record { id: 9 }]);
        assert_eq!(result.expect("a record should remain").id, 9);
    }
}
