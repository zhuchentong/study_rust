#[cfg(test)]
mod tests {
    use std::{sync::mpsc, thread};

    #[test]
    fn test_mpsc_01() {
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();

        thread::spawn(move || {
            let val = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for v in val {
                tx.send(v).unwrap();
            }
        });

        thread::spawn(move || {
            let val = vec![
                String::from("hi1"),
                String::from("from1"),
                String::from("the1"),
                String::from("thread2"),
            ];

            for v in val {
                tx1.send(v).unwrap();
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
}
