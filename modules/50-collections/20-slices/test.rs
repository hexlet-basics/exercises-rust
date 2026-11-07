#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::sum_without_first(&[]), 0);
        assert_eq!(index::sum_without_first(&[5, -2, 8]), 6);
        assert_eq!(index::sum_without_first(&[-5]), 0);
        let values = [10, 20, 30, 40];
        assert_eq!(index::sum_without_first(&values), 90);
        assert_eq!(index::sum_without_first(&values[1..3]), 30);
        assert_eq!(values, [10, 20, 30, 40]);
        let vector = vec![5, -2, 8];
        assert_eq!(index::sum_without_first(&vector), 6);
        assert_eq!(vector, vec![5, -2, 8]);
    }
}
