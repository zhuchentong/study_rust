type Kilometers = i32;
type Result<T> = std::io::Result<T>;

fn main() {
    let x = 10;
    let y: Kilometers = 20;

    println!("sum is {}", x + y);
    if let Ok(x) = test() {
        println!("the result is {x}");
    } else {
        test1()
    }
}

fn test() -> Result<i32> {
    // Ok(123)
    Err(std::io::Error::new(std::io::ErrorKind::AddrInUse, "asd"))
}

fn test1() -> ! {
    println!("somethine is wrong");
    panic!("i'm dead!!!!")
}
