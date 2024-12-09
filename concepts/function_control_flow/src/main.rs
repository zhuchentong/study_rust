fn main() {
    println!("Hello, world!");

    another_function(1, 2);
    let x = plus_one(5);
    println!("The value of x is: {}", x);
    println!("1:{x}");

    loop_1();
    loop_2();

    for_1();
    range_1();
}

fn another_function(x: i32, y: i32) -> i32 {
    println!("Hello, another_functoin!");
    println!("The value of x+y is: {}", x + y);

    // 大括号也是表达式
    let c = {
        let x = 5;
        // 结尾没有分号，则为表达式
        // 返回值是 x+1
        x + 1
    };

    c
}

// 函数需要显示声明返回类型
// 1. 可以使用return返回值
// 2. 最后一行的表达式是返回值
fn plus_one(x: i32) -> i32 {
    if x < 0 {
        return 1;
    } else {
        x + 1
    }
}

// loop
// break 停止loop, 跳出循环
// continue 停止本次循环，继续下一次循环

// loop 返回break表达式
// loop 跳出指定标签

fn loop_1() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn loop_2() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

// for ... in ...
fn for_1() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

// Range: start .. end

fn range_1() {
    for item in 1..4 {
        println!("{item}")
    }
}
