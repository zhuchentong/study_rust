fn main() {
    println!("Hello, world!");

    demo_1();

    unsafe {
        demo_2();
    }

    demo_3();

    demo_4();

    demo_5();
}

fn demo_1() {
    let mut num = 15;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }
}

unsafe fn demo_2() {
    println!("demo_2");
}

fn demo_3() {
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        // 重复借用
        // let a = (&mut values[..mid], &mut values[mid..]);
        // println!("a: {:?}", a);

        unsafe {
            (
                std::slice::from_raw_parts_mut(ptr, mid),
                std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 5);
    println!("a: {:?}", a);
    println!("b: {:?}", b);
}

unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

fn demo_4() {
    println!("abs(-10): {}", abs(-10));
}

fn demo_5() {
    static mut COUNT: u32 = 0;

    unsafe fn add_to_count(inc: i32) {
        unsafe { COUNT += inc as u32 };
    }

    unsafe { add_to_count(10) };
    unsafe { println!("COUNTER: {}", *(&raw const COUNT)) }
}
