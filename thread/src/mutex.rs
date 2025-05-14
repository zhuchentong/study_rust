#[cfg(test)]
mod tests {
    use std::sync::Arc;

    #[test]
    fn test_mutex_01() {
        use std::sync::Mutex;

        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);
    }

    #[test]
    fn test_mutex_02() {
        use std::sync::Mutex;

        let counter = Arc::new(Mutex::new(0));
        let mut handlers = vec![];

        for _ in 0..20 {
            let c = counter.clone();
            let handler = std::thread::spawn(move || {
                let mut num = c.lock().unwrap();
                *num += 1;
            });

            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}
