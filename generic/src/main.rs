
// 泛型函数
fn largest<T: PartialOrd>(list: &[T])-> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = &item;
        }
    }

    largest
}

// 泛型结构体
struct Point<T> {
    x: T,
    y: T
}

// 泛型枚举
enum Option<T> {
    Some(T),
    None
}

fn main() {
  let result =  largest(&[1, 2, 3, 4]);
  println!("The largest number is {}", result);

  let result =  largest(&['a','b','c','d']);
  println!("The largest char is {}", result);

  let point1 = Point { x: 1.0, y: 2.0 };
  let point2 = Point {x: 1,y:2};

  let option1 = Option::Some(1);

}
