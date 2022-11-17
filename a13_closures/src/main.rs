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
    let a = Some(1);
    let a_is_some = a.is_some();
    println!("a_is_some:{:?}", a_is_some);
    let a_is_none = a.is_none();
    println!("a_is_none:{:?}", a_is_none);
    let a_mapped = a.map(|num| num + 1);
    println!("mapped:{:?}", a_mapped);
    let a_filtered = a.filter(|num| num == &1);
    println!("filtered:{:?}", a_filtered);
    let a_or_else = a.or(Some(5));
    println!("a_or_else {:?}", a_or_else);
    let unwrapped = a.unwrap_or(0);
    println!("unwrapped {:?}", unwrapped);
}
