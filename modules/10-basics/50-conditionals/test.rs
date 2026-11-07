#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::access_level(18, true, false), 2);
        assert_eq!(index::access_level(17, true, false), 1);
        assert_eq!(index::access_level(18, false, false), 1);
        assert_eq!(index::access_level(120, true, false), 2);
        assert_eq!(index::access_level(0, false, false), 1);
        assert_eq!(index::access_level(25, true, true), 0);
        assert_eq!(index::access_level(0, false, true), 0);
    }
}
