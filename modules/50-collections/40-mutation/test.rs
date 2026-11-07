#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        let mut values = vec![3, -1, 2, -5];
        index::normalize(&mut values);
        assert_eq!(values, vec![0, 0, 2, 3]);
        let mut array = [5, 0, 5, 1];
        index::normalize(&mut array);
        assert_eq!(array, [0, 1, 5, 5]);
        let mut negative = [-3, -2];
        index::normalize(&mut negative);
        assert_eq!(negative, [0, 0]);
        let mut empty = [];
        index::normalize(&mut empty);
        assert_eq!(empty.len(), 0);
    }
}
