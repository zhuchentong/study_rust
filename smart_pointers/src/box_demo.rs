#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn box_demo() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("list = {:?}", list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_demo() {
        box_demo();
    }
}
