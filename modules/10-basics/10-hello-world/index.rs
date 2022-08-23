pub fn greet() -> String {
    // BEGIN
    String::from("Hello, World!")
    //END
}

#[allow(dead_code)]
pub fn main() {
    println!("{}", greet());
}
