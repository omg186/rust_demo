// # A8

// ## Topic: Organizing similar data using structs

// ## Requirements

// 1. Print th flavor of a drink and it's fluid ounces

// ## Notes

// 1. Use an enum to create different flavors of drinks
pub enum Flavor {
    Sweet,
}
// 2. Use a struct to store drink flavor and fluid ounce information
pub struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}
// 3. Use a function to print out the drink flavor and ounces
pub fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sweet => println!("Sweet"),
    }
    println!("oz:{:?}", drink.fluid_oz)
}
// 4. Use a match expression to print the drink flavor

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sweet = Drink {
            flavor: Flavor::Sweet,
            fluid_oz: 2.1,
        };
        print_drink(sweet);
    }
}
