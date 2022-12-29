use core::slice;

static mut HELLO_WORLD: &str = "hello,world!";

fn main() {
    let mut num = 5;

    let a = &num as *const i32;
    let b = &mut num as *mut i32;

    unsafe fn test() {
        println!("this is unsafe");
    }

    unsafe {
        println!("{}", *a);
        println!("{}", *b);
        *b = 1;
        println!("{}", *a);
        println!("{}", *b);

        test();

        println!("{HELLO_WORLD}");
        HELLO_WORLD = "xxx";
        println!("{HELLO_WORLD}");
    }
    let mut slice = vec![1, 2, 3, 4, 5, 6];
    let r = split_at_mut(&mut slice, 4);

    println!("{r:?}");
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
