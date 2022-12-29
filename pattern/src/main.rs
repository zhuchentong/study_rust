fn main() {
    let a = Some(5);
    match a {
        Some(10) => println!("a is 5"),
        Some(y) => println!("a match {y}"),
        _ => println!("not match any"),
    }

    let a = Some(10);
    match a {
        Some(20) => println!("a is 20"),
        Some(0..=20) => println!("a in 0..=20"),
        _ => println!("not match any"),
    }

    struct Test {
        a: i32,
        b: i32,
    }

    let test = Test { a: 11, b: 22 };

    let Test { a, b } = test;

    println!("a is {a},b is {b}");

    match test {
        Test { a, b: 20 } => println!("a is {a},b = 20"),
        Test { a: 11, b } => println!("a = 11,b is {b}"),
        Test { a, b } => println!("i don't know ,but a is {a},b is {b}"),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        Change(Test),
    }

    let msg = Message::Change(Test { a: 30, b: 10 });

    match msg {
        Message::Quit => println!("msg is quit"),
        Message::Move { x: 10, y } => println!("msg is Move,and x === 10,y is {b}"),
        Message::Move { x, y } => println!("msg is Move,x is {x},y is {y}"),
        Message::Write(s) => println!("msg is write,s is {s}"),
        Message::Change(Test { a: 30, b }) => println!("msg is change,and x === 30 ,b is {b}"),
        Message::Change(Test { a, b }) => println!("msg is change,a is {a},b is {b}"),
    }

    let data = (1, 2, 3, 4, 4);

    match data {
        (2, ..) => println!("start is 1"),
        (3, ..) => println!("start is 5"),
        (.., 4) => println!("end is 4"),
        (.., 5) => println!("end is 5"),
        _ => println!("not match "),
    }

    match data {
        (2, ..) => println!("start is 1"),
        (3, ..) => println!("start is 5"),
        (.., end @ 1..=4) => println!("end is 2|3|4,active is {end}"),
        (.., x) if x > 4 => println!("end large 4"),
        _ => println!("not match "),
    }
}
