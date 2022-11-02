pub fn first_name() -> String {
    let first = "hz".to_string();
    println!("{:?}", first);
    first
}
pub fn last_name() -> String {
    let last = "d".to_string();
    println!("{:?}", last);
    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fist_name_should_work() {
        let result = first_name();
        assert_eq!(result, "hz");
    }
    #[test]
    fn last_name_should_work() {
        let result = last_name();
        assert_eq!(result, "d")
    }
}
