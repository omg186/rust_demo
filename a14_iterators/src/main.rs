use std::vec;

fn main() {
    println!("Hello, world!");
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let new_numbers: Vec<_> = numbers.iter().map(|num| num + 1).collect();
    println!("new_numbers={:?}", new_numbers);
}
