use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    ops::Deref,
    rc::Rc,
};

use List::{Cons, Nil};

fn main() {
    // 递归引用
    let a = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:?}", a);

    // Deref Trait
    let b = MyBox::new(1);
    let c = MyBox::new(2);
    assert_eq!(1, *b);
    assert_eq!(2, *c);
    drop(b);

    let value = Rc::new(RefCell::new(5));

    let d = Rc::new(RcList::Cons(
        Rc::clone(&value),
        Rc::new(RcList::Cons(
            Rc::new(RefCell::new(10)),
            Rc::new(RcList::Nil),
        )),
    ));

    let e = RcList::Cons(Rc::new(RefCell::new(1)), Rc::clone(&d));
    let f = RcList::Cons(Rc::new(RefCell::new(2)), Rc::clone(&d));

    println!("{e:?}");
    println!("{f:?}");

    *value.borrow_mut() += 100;

    println!("{e:?}");
    println!("{f:?}");
}

// Box<T>
// Rc<T>
// RefCell<T> Ref<T> RefMut<T>
// Cell<T> 复制
// Mutex<T> 跨线程
// Weak<T> 弱引用

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum RcList {
    Cons(Rc<RefCell<i32>>, Rc<RcList>),
    Nil,
}

struct MyBox<T: Display>(T);

impl<T: Display> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        MyBox(value)
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("drop :{}", &self.0)
    }
}
