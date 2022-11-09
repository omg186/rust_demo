pub struct GroceryItem {
    name: String,
    quantity: i32,
}

pub fn find_quantity(name: &str, groceries: &Vec<GroceryItem>) -> Option<i32> {
    for grocery in groceries {
        if grocery.name == name {
            return Some(grocery.quantity);
        }
    }
    None
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn find_quantity_should_work() {
        let groceries = vec![
            GroceryItem {
                name: "bananas".to_owned(),
                quantity: 4,
            },
            GroceryItem {
                name: "eggs".to_owned(),
                quantity: 12,
            },
            GroceryItem {
                name: "apple".to_owned(),
                quantity: 10,
            },
        ];
        let result = find_quantity("bananas", &groceries);
        assert_eq!(Some(4), result);
        let result = find_quantity("apple", &groceries);
        assert_eq!(Some(10), result);
        let result = find_quantity("bread", &groceries);
        assert_eq!(None, result);
    }
}
