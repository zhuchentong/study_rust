fn main() {
    let mut s = String::from("hello");
    // string 切片类型为： &str
    let hello = &s[..5];
    println!("{hello}");
    s.push_str(" world!");
    println!("{s}");
}
