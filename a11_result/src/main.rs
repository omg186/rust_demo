use crate::{
    demo::{try_purchase, Customer},
    demo1::{try_access, Employee, Position, Status},
};
mod demo;
mod demo1;
fn main() -> Result<(), String> {
    println!("Hello, world!");

    let custom = Customer { age: 22 };
    try_purchase(&custom)?;
    let employee = Employee {
        position: Position::Managers,
        status: Status::Active,
    };
    try_access(&employee)?;
    let employee = Employee {
        position: Position::AssemblyTechnicians,
        status: Status::Terminated,
    };
    try_access(&employee)?;
    Ok(())
}
