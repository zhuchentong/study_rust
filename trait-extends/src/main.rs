use std::{fmt::Debug, ops::Add};

struct Test {
    i: u32,
    value: Vec<i32>,
}

trait GoTo {
    type Item: Debug;

    fn goto(&mut self) -> Self::Item;

    fn test() {
        println!("this is test in GOTO");
    }

    fn todo(&self) {
        println!("my name is todo");
    }
}

trait RunTo: GoTo {
    fn run(&self) {
        self.todo();
        println!("i do it for run ");
    }
}

trait JumpTo: RunTo {
    fn jump(&self) {
        self.run();
        println!("i do it for jump ");
    }
}

impl GoTo for Test {
    type Item = u32;

    fn goto(&mut self) -> Self::Item {
        while let Some(_) = self.value.pop() {
            self.i += 1;
        }

        self.i
    }
}

impl Test {
    fn test() {
        println!("this is test in Test");
    }
}

impl RunTo for Test {}
impl JumpTo for Test {}

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

fn main() {
    let mut goto = Test {
        i: 5,
        value: vec![1, 2, 3, 4, 5, 6],
    };

    println!("result is {}", goto.goto());
    println!("{:?} {}", goto.value, goto.i);

    println!("1m + 100mm = {:?}", Millimeters(100) + Meters(1));

    Test::test();
    <Test as GoTo>::test();

    goto.jump();
}

// fn generic<T: ?Sized>(a: &T) {}
