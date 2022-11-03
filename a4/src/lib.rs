// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
#[derive(Debug)]
pub enum Colors {
    Red,
    Blue,
}
// * Use a function to print the color name
// * The function must use the enum as a parameter
pub fn print_color_name(color: Colors) {
    println!("{:?}", color);
}
// name to print
#[cfg(test)]
mod tests {
    use crate::{print_color_name, Colors};

    #[test]
    fn it_work() {
        print_color_name(Colors::Red);
    }
}
