use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    let counter = Arc::new(Mutex::new(1));
    let counter1 = Arc::clone(&counter);
    let counter2 = Arc::clone(&counter);

    thread::spawn(move || {
        let val = vec![
            String::from("hello lee1!"),
            String::from("hello lee2!"),
            String::from("hello lee3!"),
        ];

        for mut v in val {
            let mut n = counter1.lock().unwrap();
            v.push_str(&n.to_string());
            tx.send(v).unwrap();
            *n += 1;
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let val = vec![
            String::from("hello zct1!"),
            String::from("hello zct2!"),
            String::from("hello zct3!"),
            String::from("hello zct4!"),
        ];

        for mut v in val {
            let mut n = counter2.lock().unwrap();
            v.push_str(&n.to_string());
            tx1.send(v).unwrap();
            *n += 1;
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("get: {}", received);
    }
}
