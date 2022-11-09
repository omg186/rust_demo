struct Customer {
    age: Option<i32>,
    email: String,
}

fn main() {
    println!("Hello, world!");
    let mark = Customer {
        age: Some(12),
        email: "aaabbbcc.@email.com".to_owned(),
    };
    match mark.age {
        Some(age) => println!("customer is {:?} years old {:?}", age, mark.email),
        None => println!("customer age not provided"),
    }
}
