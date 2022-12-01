use async_std::task::sleep;
use futures::{executor::block_on, pin_mut, Future, FutureExt};
use std::time::Duration;

fn main() {
    let race_future = race_tasks();
    block_on(race_future);
    let block_future = async_blocks();
    block_on(block_future);
    let borrow_future = async_borrow_x();
    block_on(borrow_future);
    let future = async_main();
    block_on(future);
    println!("this is the end line");
}

/**
 * 使用Select实现竞态并发
 */
async fn race_tasks() {
    let t1 = test_async1_sub1().fuse();
    let t2 = test_async1_sub2().fuse();

    pin_mut!(t1, t2);

    loop {
        futures::select! {
            () = t1 => println!("任务1率先完成"),
            () = t2 => println!("任务2率先完成"),
            complete=>{
                println!("任务全部完成");
                break;
            },
            default=>panic!(),
        }
    }
}

async fn borrow_x(x: &u8) -> u8 {
    *x
}

/**
 * 环境只可以通过Move转移生命周期都一个async中
 */
async fn async_blocks() {
    let my_string = "foo".to_string();

    let future_one = async {
        // ...
        println!("{my_string}");
    };

    let future_two = async {
        // ...
        println!("{my_string}");
    };

    // 运行两个 Future 直到完成
    let ((), ()) = futures::join!(future_one, future_two);
}

/**
 * 通过aysnc move可以转移变量生命周期
 */
fn async_borrow_x() -> impl Future<Output = u8> {
    let x = 5;

    async move {
        println!("borrow x in lifetime");
        borrow_x(&x).await
    }
}

async fn async_main() {
    let future1 = test_async1();
    let future2 = test_async2();

    futures::join!(future1, future2);
}

async fn test_async1() {
    test_async1_sub1().await;
    test_async1_sub2().await;
    sleep(Duration::from_secs(1)).await;
    println!("this is test for async1");
}

async fn test_async2() {
    println!("this is test for async2");
}

async fn test_async1_sub1() {
    sleep(Duration::from_micros(100)).await;
    println!("this is test for async1_sub1");
}

async fn test_async1_sub2() {
    println!("this is test for async1_sub2");
}
