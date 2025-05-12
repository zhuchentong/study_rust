#[cfg(test)]
mod tests {
    /// 主线程结束时，子线程也会结束
    #[test]
    fn test_thread_spawn_01() {
        for i in 0..5 {
            println!("main thread: {i}")
        }

        std::thread::spawn(|| {
            for i in 0..10 {
                println!("spawned thread: {i}")
            }
        });
    }

    /// 主线程结束时，子线程也会结束
    /// 通过使用 handle.join() 来等待子线程结束
    #[test]
    fn test_thread_spawn_02() {
        for i in 0..5 {
            println!("main thread: {i}")
        }

        let handle = std::thread::spawn(|| {
            for i in 0..10 {
                println!("spawned thread: {i}")
            }
        });

        handle.join().unwrap();
    }

    #[test]
    fn test_thread_spawn_03() {
        let mut n = 1;
        let x = n;
        for i in 0..5 {
            println!("main thread index: {i}")
        }

        let handle = std::thread::spawn(move || {
            for i in 0..10 {
                n = n + 1;
                println!("spawned thread index: {i}");
                println!("spawned thread n: {n}");
            }
        });
        n += 1;
        handle.join().unwrap();

        println!("main thread n: {n}");
        println!("main thread n: {x}");
    }
}
