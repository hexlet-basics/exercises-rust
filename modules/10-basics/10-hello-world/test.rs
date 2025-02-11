mod index;

#[allow(dead_code)]
pub fn main() {
    use index::greet;
    greet();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let cmd = std::process::Command::new("cargo")
            .arg("run")
            .output()
            .expect("`cargo run` works for basic projects");
        let out = String::from_utf8_lossy(&cmd.stdout);

        assert_eq!(out, "Hello, World!\n");
    }
}
