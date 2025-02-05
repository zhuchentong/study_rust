use std::fs;

fn open_file() -> Result<String, std::io::Error> {
    // let mut text: String = String::new();
    // let _ = fs::File::open("hello.txt")?.read_to_string(&mut text);
    // Ok(text)
    fs::read_to_string("hello.txt")
}

fn main() {
    println!("Hello, world!");

    // panic!("123123")

    // let file_result = File::open("hello.txt");

    // if let Ok(file) = file {
    //     println!("{file:?}");
    // } else {
    //     println!("Error Not Found File");
    // }

    // let a = file_result.expect("File is Not Found");
    let a = open_file();

    // let a = match file_result {
    //     Ok(file) => file,
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(file) => file,
    //             Err(e) => panic!("{e:?}"),
    //         },
    //         other => panic!("{other:?}"),
    //     },
    // };
    print!("{a:?}")
}
