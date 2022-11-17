// Topic: Map combinator
//
// Requirements
// * Given a user name, create and print out a User struct if the user exists
// Notes:
// * Use the existing find_user function to locate user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not
#[derive(Debug)]
pub struct User {
    pub user_id: i32,
    pub name: String,
}

pub fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        _ => None,
    }
}
pub fn to_user(name: &str) -> Option<User> {
    find_user(name).map(|user_id| User {
        user_id,
        name: name.to_owned(),
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_user_work() {
        let name = "sam";
        assert_eq!(Some(1), find_user(name));
    }
    #[test]
    fn should_find_user_not_found_work() {
        let name = "sam1";
        assert_eq!(None, find_user(name));
    }
    #[test]
    fn should_to_user_work() {
        let name = "sam".to_owned();
        let user = to_user(&name);
        assert!(user.is_some())
    }
    #[test]
    fn should_to_user_not_found_work() {
        let name = "sam1".to_owned();
        let user = to_user(&name);
        println!("{:?}", user);
        assert!(user.is_none())
    }
}
