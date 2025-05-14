#[cfg(test)]
mod tests {
    use std::{
        pin::{Pin, pin},
        time::Duration,
        vec,
    };

    #[test]
    fn test_async_01() {
        trpl::run(async {
            let handle = trpl::spawn_task(async {
                for i in 0..10 {
                    println!("spawned thread: {i}");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            });

            for i in 0..5 {
                println!("main thread: {i}");
                trpl::sleep(Duration::from_millis(500)).await;
            }

            handle.await.unwrap();
        })
    }

    #[test]
    fn test_async_02() {
        trpl::run(async {
            let future1 = async {
                for i in 0..10 {
                    println!("thread 1: {i}");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let future2 = async {
                for i in 0..5 {
                    println!("thread 2: {i}");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            trpl::join!(future1, future2);
        })
    }

    #[test]
    fn test_async_03() {
        trpl::run(async {
            let (tx, mut rx) = trpl::channel();

            let tx_clone = tx.clone();

            let futrue3 = async move {
                while let Some(msg) = rx.recv().await {
                    println!("msg: {msg}");
                }
            };

            let futrue1 = async move {
                let msg = vec![
                    String::from("hello"),
                    String::from("world"),
                    String::from("rust"),
                    String::from("async"),
                ];

                for m in msg {
                    tx.send(m).unwrap();
                    trpl::sleep(Duration::from_millis(2500)).await;
                }
            };

            let futrue2 = async move {
                let msg = vec![
                    String::from("111"),
                    String::from("222"),
                    String::from("333"),
                    String::from("444"),
                ];

                for m in msg {
                    tx_clone.send(m).unwrap();
                    trpl::sleep(Duration::from_millis(2500)).await;
                }
            };

            trpl::join3(futrue1, futrue2, futrue3).await;
        });
    }

    #[test]
    fn test_async_04() {
        trpl::run(async {
            let (tx, mut rx) = trpl::channel();

            let tx_clone = tx.clone();

            let futrue3 = pin!(async move {
                while let Some(msg) = rx.recv().await {
                    println!("msg: {msg}");
                }
            });

            let futrue1 = pin!(async move {
                let msg = vec![
                    String::from("hello"),
                    String::from("world"),
                    String::from("rust"),
                    String::from("async"),
                ];

                for m in msg {
                    tx.send(m).unwrap();
                    trpl::sleep(Duration::from_millis(2500)).await;
                }
            });

            let futrue2 = pin!(async move {
                let msg = vec![
                    String::from("111"),
                    String::from("222"),
                    String::from("333"),
                    String::from("444"),
                ];

                for m in msg {
                    tx_clone.send(m).unwrap();
                    trpl::sleep(Duration::from_millis(2500)).await;
                }
            });

            let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![futrue1, futrue2, futrue3];

            trpl::join_all(futures).await;
        });
    }

    #[test]
    fn test_async_05() {
        trpl::run(async {
            let future1 = async {
                println!("future1: start");
                trpl::yield_now().await;
                println!("future1: 1");
                trpl::yield_now().await;
                println!("future1: 2");
                trpl::yield_now().await;
                println!("future1: 3");
                trpl::yield_now().await;
                trpl::sleep(Duration::from_millis(500)).await;
                println!("future1: end");
            };

            let future2 = async {
                println!("future2: start");
                trpl::yield_now().await;
                println!("future2: 1");
                trpl::yield_now().await;
                println!("future2: 2");
                trpl::yield_now().await;
                println!("future2: 3");
                trpl::sleep(Duration::from_millis(500)).await;
                println!("future2: end");
            };

            trpl::race(future1, future2).await;
        })
    }
}
