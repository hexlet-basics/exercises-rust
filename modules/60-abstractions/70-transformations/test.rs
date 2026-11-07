#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        let values = vec![-2, 0, 3, 1];
        assert_eq!(index::positive_squares(&values), vec![9, 1]);
        assert_eq!(values, vec![-2, 0, 3, 1]);
        assert_eq!(index::positive_squares(&[]), Vec::<i32>::new());
        assert_eq!(index::positive_squares(&[-3, 0, -1]), Vec::<i32>::new());
        assert_eq!(index::positive_squares(&[2, 2, 1]), vec![4, 4, 1]);
        assert_eq!(index::positive_squares(&[1000]), vec![1_000_000]);
    }
}
