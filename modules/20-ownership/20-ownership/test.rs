#[allow(dead_code)]
mod index;

fn main() {
    for text in ["Rust", "", "Book shop"] {
        let original = index::keep_original(String::from(text));
        println!("{}", original);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exercise_contract() {
        let output = std::process::Command::new("cargo")
            .args(["run", "--quiet", "--manifest-path"])
            .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
            .args(["--bin", "20-ownership-20-ownership"])
            .output()
            .expect("the exercise runner should start");
        assert!(
            output.status.success(),
            "runner failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "Rust\nRust\n\n\nBook shop\nBook shop\n"
        );
    }
}
