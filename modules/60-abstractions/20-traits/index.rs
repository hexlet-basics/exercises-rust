pub trait Named {
    fn label(&self) -> String;

    fn label_length(&self) -> usize {
        self.label().len()
    }
}

pub struct Book {
    pub title: String,
}

pub struct User {
    pub name: String,
}

// BEGIN
impl Named for Book {
    fn label(&self) -> String {
        format!("Book: {}", self.title)
    }
}

impl Named for User {
    fn label(&self) -> String {
        format!("User: {}", self.name)
    }
}
// END
