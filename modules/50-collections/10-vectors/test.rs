#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::numbers_up_to(0), Vec::<i32>::new());
        assert_eq!(index::numbers_up_to(1), vec![1]);
        assert_eq!(index::numbers_up_to(3), vec![1, 2, 3]);
        let values = index::numbers_up_to(1000);
        assert_eq!(values.len(), 1000);
        assert_eq!(values[999], 1000);
    }
}
