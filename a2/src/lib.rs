// Use function to add two numbers together
pub fn sum(left: i32, right: i32) -> i32 {
    left + right
}
// Use a function to display the result
pub fn display_result(result: i32) {
    println!("result {:?}", result);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sum(2, 2);
        display_result(result);
        assert_eq!(result, 4);
    }
}
