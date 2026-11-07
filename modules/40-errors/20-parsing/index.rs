pub fn parse_quantity(text: &str) -> Result<i32, String> {
    // BEGIN
    let parsed: Result<i32, _> = text.trim().parse();
    match parsed {
        Ok(number) => {
            if (1..=1000).contains(&number) {
                Ok(number)
            } else {
                Err(String::from("out of range"))
            }
        }
        Err(_) => Err(String::from("invalid number")),
    }
    // END
}
