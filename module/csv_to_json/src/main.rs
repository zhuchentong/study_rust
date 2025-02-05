// crate:: 指从当前文件内联模块查找
// $package:: 指从lib入口模块查找
use crate::functions::read_csv;
use csv_to_json::modules_2::structs::HousePrice;
use std::fmt::Result as FmtResult;
use std::io::Result as IOResult;

fn main() {
    println!("Hello, world!");

    let x = crate::modules_2::enums::YesNo::Yes;
    let y = csv_to_json::modules_2::enums::YesNo::Yes;
    let z = HousePrice {
        price: 100,
        name: String::from("test"),
        main_road: y,
    };

    println!("{:#?},{:#?},{:#?}", x, y, z);

    read_csv(String::from(r"hello:\world"));
}

// 内联模块
mod modules_1 {}
mod functions;
mod modules_2;
mod modules_3;

mod m1 {
    pub mod m2 {
        pub fn method_1() {
            println!("m1");
        }
    }
}

mod x1 {
    mod x2 {
        fn method() {
            super::super::m1::m2::method_1();
        }
    }
}
