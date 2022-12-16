use std::io;

pub fn greet(stdout: &mut dyn io::Write) {
    // BEGIN
    writeln!(stdout, "Hello, World!").ok();
    // END
}

#[allow(dead_code)]
pub fn main() {
    greet(&mut io::stdout());
}
