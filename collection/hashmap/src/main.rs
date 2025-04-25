use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut map1 = HashMap::new();

    map1.insert(String::from("Blue"), 123);
    map1.insert(String::from("Yellow"), 456);

    println!("{:?}", map1);

    let map2 = HashMap::from([(String::from("aaa"), 333), (String::from("bbb"), 444)]);
    println!("{:?}", map2);

    let mut map3: HashMap<_, _> = vec![("xxx", 1), ("yyy", 2)].into_iter().collect();

    map3.insert("xxx", 999);
    map3.entry("yyy").or_insert(0);

    println!("{:?}", map3);
}
