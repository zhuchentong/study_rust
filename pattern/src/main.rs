fn main() {
    struct Pointer {
        x: i32,
        y: i32,
    }

    let a = Pointer { x: 1, y: 2 };

    match a {
        Pointer { x: x @ 1..4, y } => println!("x: {}, y: {}", x, y),
        _ => println!("not match"),
    }
}
