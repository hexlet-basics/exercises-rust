#[allow(dead_code)]
mod index;

fn main() {}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn exercise_contract() {
        let mut product = index::Product {
            price: 200,
            count: 3,
        };
        assert!(product.sell_one());
        assert_eq!(product.count, 2);
        assert_eq!(product.price, 200);
        assert!(product.sell_one());
        assert!(product.sell_one());
        assert_eq!(product.count, 0);
        assert!(!product.sell_one());
        assert_eq!(product.count, 0);
        let mut empty = index::Product { price: 0, count: 0 };
        assert!(!empty.sell_one());
        assert_eq!(empty.count, 0);
        let borrowed = &mut product;
        assert!(!borrowed.sell_one());
    }
}
