use std::time::Duration;

fn main() {
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
                trpl::sleep(Duration::from_millis(500)).await;
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
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join!(futrue1, futrue2, futrue3);
    });
}
