#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        assert_eq!(index::emphasize(String::from("Rust")), "Rust!!");
        assert_eq!(index::emphasize(String::new()), "!!");
        assert_eq!(index::emphasize(String::from("done!")), "done!!!");
    }
}
