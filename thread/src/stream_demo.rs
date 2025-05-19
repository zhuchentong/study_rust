#[cfg(test)]
mod tests {
    use trpl::StreamExt;

    #[test]
    fn test_stream_01() {
        trpl::run(async {
            let v1 = vec![1, 2, 3, 4, 5, 6];
            let v1_iter = v1.iter().map(|x| x * 2);
            let mut steam_iter = trpl::stream_from_iter(v1_iter);

            while let Some(val) = steam_iter.next().await {
                println!("Got: {}", val);
            }
        })
    }
}
