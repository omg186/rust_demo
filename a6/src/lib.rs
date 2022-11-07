pub fn tuples_fn() {
    let coord = (2, 3);
    println!("{:?},{:?}", coord.0, coord.1);
    let (x, y) = coord;
    println!("{:?},{:?}", x, y);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_work() {
        tuples_fn();
    }
}
