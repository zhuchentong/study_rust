fn main() {
    // region 1.可变变量与不可变变量
    // 默认声明的变量为不可变变量
    println!("---可变变量与不可变变量---");
    let some_number = 42;
    // 使用mut声明可变变量
    let mut another_number = 123;

    println!(
        "some_number is {} and another_number is {}",
        some_number, another_number
    );

    // some_number = 45;
    another_number = 46;

    println!(
        "some_number is {} and another_number is {}",
        some_number, another_number
    );
    // endregion

    // region 2.声明常量
    // 使用const声明常量
    // 声明常量必须显示标注类型
    println!("---声明常量---");
    const SPECIAL_NUMBER: i32 = 100;
    const SPECIAL_ANOTHER_NUMBER: u32 = 100 * 100 * 100;

    println!("SPECIAL_NUMBER is {}", SPECIAL_NUMBER,);
    println!("SPECIAL_ANOTHER_NUMBER is {}", SPECIAL_ANOTHER_NUMBER);
    // endregion

    // region 3.变量遮蔽
    // 变量遮蔽是同名变量会自动覆盖之前的变量重新进行定义
    // 同时也可以设置新的变量类型
    println!("---变量遮蔽---");
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The inner value of x is {}", x);
    }
    let x = (x + 1).to_string();
    println!("The outer value of x is {}", x);
    // endregion

    // 数据类型：标量类型，符合类型

    // 标量类型：
    // Integer 整数
    // Floating Point 浮点
    // Boolean 布尔
    // Character 字符

    // 整数: i8-i128/u8-u128, isize,usize ,default:(i32)
    // isze/usize 使用操作系统（32/64）
    // 98_222 使用 _ 增加可读性

    // 浮点类型: f32,f64

    // Boolean: true,false

    // 字符类型: char （单字符）

    // 复合类型: Tuple,Array
    //  Tuple 元组 固定长度 类型可不同
    //  Array 数组 固定长度 类型相同
    println!("---复合类型---");
    println!("---元组---");
    let tuple_1 = (1, 2, String::from("test"));
    let tuple_2: (i32, i8, i64) = (1, 2, 3);

    // 使用模式匹配可以从 元组中 取值
    let (x, y, z) = tuple_2;

    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("tuple_1 is {},{},{}", tuple_1.0, tuple_1.1, tuple_1.2);

    let array_1 = [1, 2, 3];
    let array_2: [i32; 3] = [2, 3, 4];
    let array_3 = [3; 6];

    println!(
        "array_1 is {:?}; array_2 is {:?}; array_3: {:?}",
        array_1, array_2, array_3
    );

    const TTT: f32 = 3.4028236;
    println!("ttt:{}", TTT);
}
