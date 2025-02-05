fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    v1.push(3);
    v1.push(4);
    v1.push(5);

    println!("{:?}", v1);
    println!("{:?}", v2);

    let a = if let Some(v) = v2.get(12) { v } else { &4 };

    println!("value is {a}");

    for x in &v2 {
        let c = *x + 1;
        println!("{c}");
    }

    for y in &mut v1 {
        *y = *y * 2;
    }

    println!("{:?}", v1);

    let mut t1 = v1.iter();

    while if let Some(_) = &t1.next() {
        true
    } else {
        false
    } {
        println!("1:{:?}", t1);
    }
}
