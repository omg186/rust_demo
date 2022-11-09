use crate::demo::{try_purchase, Customer};

mod demo;
fn main() -> Result<(), String> {
    println!("Hello, world!");

    let custom = Customer { age: 22 };
    try_purchase(&custom)?;
    Ok(())
}
