fn main() {
    println!("Hello, world!");

    let a = Box::new(123);
    // 拷贝数据
    let b = a.clone();
    // 转移所有权
    let c = a;

    println!("{}", b);
    println!("{}", c);

    borrow_1();
    borrow_3();
    reference()
}

// fn test_1() {
//     let s = String::from("hello");
//     let s2;
//     let b = false;
//     if b {
//         s2 = s;
//     }

//     println!("{}", s);
// }

// 引用就是没有所有权的指针
fn borrow_1() {
    let m1 = String::from("hello");
    let m2 = String::from("world");
    println!("{:p}", &m1);
    greet(&m1, &m2);
    let s = format!("{} {}", m1, m2);
    println!("{}", s);
}

// fn borrow_2() {
//     let mut m1 = vec![1, 2, 3];
//     let num = &m1[2];
//     m1.push(3);
//     println!("{:?}", *m1)
// }

fn borrow_3() {
    let mut m1 = vec![1, 2, 3];
    let m2 = &mut m1[2];
    let m3 = &*m2;
    // *m2 = 4;

    println!("{:?},{:?}", m2, m3);
}

// fn borrow_4{
//     let mut s = String::from("hello");
//     let s2 = &s;
//     let s3 = &mut s;
//     s3.push_str(" world");
//     println!("{}", s2)
// }

fn greet(g1: &String, g2: &String) {
    println!("{} {}", g1, g2);
    let address_in_g1 = g1 as *const String;
    println!("{:p}", address_in_g1);
    println!("{:p}", &g1);
}

// 解引用
fn reference() {
    let mut m1 = Box::new(123);
    println!("{}", *m1);
    *m1 += 1;
    println!("{}", *m1);
    let r1: &mut Box<i32> = &mut m1;
    **r1 += 1;
    println!("{}", r1);
    let m2 = Box::new("123");
    let m2_len = m2.len();
    println!("{}", m2_len);
    println!("{}", str::len(&m2));
}
