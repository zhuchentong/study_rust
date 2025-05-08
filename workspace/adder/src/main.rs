use add_one;
use add_two;

fn main() {
    let one_result = add_one::add_one(1);
    let two_result = add_two::add_two(1);
    println!("one_result: {}", one_result);
    println!("two_result: {}", two_result);
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_one_works() {
        let result = add_one::add_one(2);
        assert_eq!(result, 3);
    }
    #[test]
    fn add_two_works() {
        let result = add_two::add_two(2);
        assert_eq!(result, 4);
    }
}
