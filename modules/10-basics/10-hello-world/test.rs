#[allow(dead_code)]
mod index;

fn main() {
    index::greet();
}

#[cfg(test)]
mod tests {

    #[test]
    fn exercise_contract() {
        let output = std::process::Command::new("cargo")
            .args(["run", "--quiet", "--manifest-path"])
            .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
            .args(["--bin", "10-basics-10-hello-world"])
            .output()
            .expect("the exercise runner should start");
        assert!(
            output.status.success(),
            "runner failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello, World!\n");
    }
}
