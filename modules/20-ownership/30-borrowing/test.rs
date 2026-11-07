#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::larger(&7, &-2), 7);
        assert_eq!(index::larger(&-10, &-2), -2);
        assert_eq!(index::larger(&0, &0), 0);
        assert_eq!(index::larger(&-1_000_000, &1_000_000), 1_000_000);
        let left = 3;
        let right = 8;
        assert_eq!(index::larger(&left, &right), 8);
    }
}
