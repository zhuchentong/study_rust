use core::fmt;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("EMAIL"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("EMAIL1"),
        ..user1
    };

    let color1 = Color(255, 1, 1);

    let subject = AlwaysEqual;

    println!("{:#?}", user2);

    let mut point = Point { x: 1, y: 1 };
    let x = &mut point.x;

    point.y = 3;
    println!("x:{},y:{}", x, point.y);

    user2.hello();

    println!("{:#?}", User::test());
    println!("{}", format!("data is {user2}"));
}

// derive trait
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{})", self.username, self.sign_in_count)
    }
}

impl User {
    fn hello(&self) {
        println!("my name is {}", self.username);
    }

    fn test() -> Self {
        Self {
            active: false,
            username: String::from("123"),
            email: String::from("xxx"),
            sign_in_count: 4,
        }
    }
}

// Tuple Struct
struct Color(i32, i32, i32);

// Unit-Like Struct
struct AlwaysEqual;

struct Point {
    x: i32,
    y: i32,
}
