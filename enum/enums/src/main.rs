use rand::prelude::*;
fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{},{}", four, six);
    println!("{}", IpAddrKind::test().unwrap());

    // 表示a可能为空  Option<&str>
    let mut a = Some("123");

    println!("a:{}", a.unwrap());
    if rand::random() {
        a = None;
    }

    // 模式匹配
    match &a {
        Some(s) => println!("a:{}", s),
        None => println!("a is None"),
        _ => println!("a is other"),
    }

    if let Some(s) = a {
        println!("a is {}", s);
    } else {
        println!("a is not 123");
    }
}

// 枚举可以持有数据,数据类型可以使元组或结构体
enum IpAddrKind {
    V4,
    V6,
}

impl IpAddrKind {
    fn test() -> Option<IpAddrKind> {
        Some(IpAddrKind::V4)
    }
}

impl std::fmt::Display for IpAddrKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpAddrKind::V4 => write!(f, "V4"),
            IpAddrKind::V6 => write!(f, "V6"),
        }
    }
}

// enum Option<T> {
//     None,
//     Some(T),
// }
