#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        let left = String::from("cat");
        let right = String::from("elephant");
        let result = index::shorter(&left, &right);
        assert_eq!(result, "cat");
        assert!(std::ptr::eq(result.as_ptr(), left.as_ptr()));
        let multibyte = String::from("éé");
        let ascii = String::from("abc");
        let selected = index::shorter(&multibyte, &ascii);
        assert_eq!(selected, "abc");
        assert!(std::ptr::eq(selected.as_ptr(), ascii.as_ptr()));
        let equal_left = String::from("aa");
        let equal_right = String::from("bb");
        let selected = index::shorter(&equal_left, &equal_right);
        assert_eq!(selected, "aa");
        assert!(std::ptr::eq(selected.as_ptr(), equal_left.as_ptr()));
        assert_eq!(index::shorter("long", "x"), "x");
        assert_eq!(index::shorter("", "abc"), "");
        assert_eq!(index::shorter("abc", ""), "");
        let left = String::from("é");
        let right = String::from("ab");
        let result = index::shorter(&left, &right);
        assert_eq!(result, "é");
        assert!(std::ptr::eq(result.as_ptr(), left.as_ptr()));
    }
}
