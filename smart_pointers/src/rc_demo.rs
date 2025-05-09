use std::rc::Rc;

#[derive(Debug)]
pub enum List<T> {
    Nil,
    Cons(T, Rc<List<T>>),
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        println!("Dropping List containing");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc_demo() {
        let a = Rc::new(List::Cons(3, Rc::new(List::Cons(4, Rc::new(List::Nil)))));
        let b = List::Cons(1, Rc::clone(&a));
        let c = List::Cons(2, Rc::clone(&a));
        println!("b = {:?}", b);
        println!("c = {:?}", c);
    }
}
