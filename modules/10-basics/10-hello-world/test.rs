mod index;

#[cfg(test)]
mod tests {
    use index::greet;

    #[test]
    fn test_greet() {
        assert_eq!(greet(), "Hello, World!");
    }

}
