pub struct Test {
    score: i32,
}
pub fn my_vec() {
    let my_vector = vec![Test { score: 99 }, Test { score: 87 }];
    for test in my_vector {
        println!("score = {:?}", test.score);
    }
}

// 1. Print 10, 20 , "thirty" , and 40 in a loop
// 2. Print the total number of elements in a vector

// // Notes:

// 1. Use a vector to store 4 numbers

// 2. Iterate through the vector using a for..in loop

// 3. Determine whether to print the number or print "thirty" inside the loop
// 4. Use the .len() function to print the number of elements in a vector
pub fn loo_vec(numbers: &Vec<i32>) {
    for number in numbers {
        match number {
            30 => println!("thirty"),
            _ => println!("{:?}", number),
        };
    }
    println!("len:{:?}", numbers.len());
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        my_vec();
        let my_numbers = vec![10, 20, 30, 40];
        loo_vec(&my_numbers);
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
