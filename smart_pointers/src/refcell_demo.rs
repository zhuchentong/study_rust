use std::cell::RefCell;

pub struct MyBox<T> {
    pub value: RefCell<T>,
}

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox {
            value: RefCell::new(x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MyBox;

    #[test]
    fn test_refcell_demo() {
        let a = MyBox::new(5);
        let b = MyBox::new(6);
        assert_eq!(*a.value.borrow(), 5);
        assert_eq!(*b.value.borrow(), 6);

        *a.value.borrow_mut() = 10;
        assert_eq!(*a.value.borrow(), 10);
        *b.value.borrow_mut() = 20;
        assert_eq!(*b.value.borrow(), 20);
    }
}
