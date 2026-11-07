pub struct Product {
    pub price: i32,
    pub count: i32,
}

impl Product {
    pub fn sell_one(&mut self) -> bool {
        // BEGIN
        if self.count > 0 {
            self.count -= 1;
            true
        } else {
            false
        }
        // END
    }
}
