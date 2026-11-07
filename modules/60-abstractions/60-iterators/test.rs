#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        let mut numbers = 3..7;
        assert_eq!(index::take_two(&mut numbers), (Some(3), Some(4)));
        assert_eq!(numbers.next(), Some(5));
        let mut single = 7..8;
        assert_eq!(index::take_two(&mut single), (Some(7), None));
        assert_eq!(single.next(), None);
        let mut empty = 5..5;
        assert_eq!(index::take_two(&mut empty), (None, None));
        let mut negative = -2..2;
        assert_eq!(index::take_two(&mut negative), (Some(-2), Some(-1)));
        assert_eq!(negative.next(), Some(0));
    }
}
