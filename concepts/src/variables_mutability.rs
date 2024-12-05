
pub fn main() {
    // 声明变量 - 不可变
    let some_number = 1;

    // 声明变量 - 可变
    let mut another_number = 3;

    another_number = 4;

    println!("The value of some number is {}", some_number);
    println!("The value of another number is {}", another_number);
}