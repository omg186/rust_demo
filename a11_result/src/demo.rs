// Use a struct to store at least the age of a customer
pub struct Customer {
    pub age: i32,
}
// Use a function to determine if a customer can make a restricted purchase

pub fn try_purchase(customer: &Customer) -> Result<(), String> {
    if customer.age < 21 {
        return Err("客户年龄未到21岁，不符合购买条件".to_owned());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_work() -> Result<(), String> {
        let custom = Customer { age: 22 };
        try_purchase(&custom)?;
        Ok(())
    }
    #[test]
    fn it_err_work() {
        let custom = Customer { age: 20 };
        let result = try_purchase(&custom);
        assert!(result.is_err());
    }
}
