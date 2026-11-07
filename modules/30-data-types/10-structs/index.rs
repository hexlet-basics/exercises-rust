#[derive(Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub active: bool,
}

pub fn rename_user(user: User, new_name: String) -> User {
    // BEGIN
    let mut user = user;
    user.name = new_name;
    user
    // END
}
