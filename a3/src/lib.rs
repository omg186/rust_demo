pub fn display_message(visible: bool) -> String {
    if visible {
        "hello".to_string()
    } else {
        "goodbye".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_true_should_works() {
        let my_bool = true;
        assert!(display_message(my_bool).contains("hello"));
    }
    #[test]
    fn it_false_should_works() {
        let my_bool = false;
        assert!(display_message(my_bool).contains("goodbye"));
    }
}