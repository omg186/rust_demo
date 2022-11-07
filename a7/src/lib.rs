pub struct GroceryItem {
    quantity: i32,
    id: i32,
}

impl GroceryItem {
    pub fn display_quantity(&self) {
        println!("{:?}", self.quantity);
    }
    pub fn display_id(&self) {
        println!("{:?}", self.id);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        let grocery_item = GroceryItem {
            quantity: 20,
            id: 1,
        };
        grocery_item.display_quantity();
        grocery_item.display_id();
        // assert_eq!(result, 4);
    }
}
