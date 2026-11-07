// Iterator adapters are introduced later, so this lesson uses an explicit search.
#[allow(dead_code, clippy::manual_find)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::first_positive([-2, 0, 7, 3, 9]), Some(7));
        assert_eq!(index::first_positive([0, -1, -8, 0, -2]), None);
        assert_eq!(index::first_positive([2, 9, 8, 7, 6]), Some(2));
        assert_eq!(index::first_positive([-1, -2, -3, -4, 5]), Some(5));
    }
}
