mod index;

#[cfg(test)]
mod tests {
    use index::greet;

    #[test]
    fn test_greet() {
        let mut stdout = Vec::new();
        greet(&mut stdout);

        assert_eq!(stdout, b"Hello, World!\n");
    }

}
