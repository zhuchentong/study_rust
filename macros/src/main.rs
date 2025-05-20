use hello_macro_derive::HelloMacro;

pub trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    println!("Hello, world!");
    test1!("Hello, world!");

    Pancakes::hello_macro();
}

#[macro_export]
macro_rules! test1 {
    ($($arg:tt)*) => {
        println!("test1: {}", format_args!($($arg)*));
    };
}
