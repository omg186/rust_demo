use crate::demo::to_user;

mod demo;
// fn maybe_word() -> Option<String> {}

fn main() {
    println!("Hello, world!");

    let name = "sam".to_owned();
    let user = to_user(&name);
    match user {
        Some(user) => print!("{:?}", user),
        None => println!("user not found"),
    }
    let user = to_user("sam1");
    match user {
        Some(user) => print!("{:?}", user),
        None => println!("user not found"),
    }
    // let word_length = maybe_word().map(|word| word.len()).map(|len| len * 2);
}
