// BEGIN
pub fn max_ref<T: Ord>(values: &[T]) -> Option<&T> {
    let mut largest = values.first()?;
    for value in values {
        if value > largest {
            largest = value;
        }
    }
    Some(largest)
}
// END
