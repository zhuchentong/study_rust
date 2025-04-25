fn main() {
    let str1 = "test1".to_string();
    println!("Hello, world! {}", &str1);

    let s1 = String::from("111");
    let s2 = String::from("222");
    let s3 = String::from("333");

    let s4 = format!("{s1}-{s2}-{s3}");
    let s5 = s1 + "-" + &s2 + "-" + &s3;

    println!("s4: {s4}");
    println!("s5: {s5}");

    for c in s5.chars() {
        println!("c: {c}")
    }

    for c in s5.bytes() {
        println!("c: {c}")
    }
}
