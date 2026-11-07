use std::collections::HashMap;

pub fn update_price(prices: &mut HashMap<String, i32>, title: &str, price: i32) -> Option<i32> {
    // BEGIN
    match prices.get_mut(title) {
        Some(current) => {
            let previous = *current;
            *current = price;
            Some(previous)
        }
        None => None,
    }
    // END
}
