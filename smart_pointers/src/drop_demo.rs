pub struct MyBox<T>(T);

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox containing");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_demo() {
        let b = MyBox(5);
        println!("b = {}", b.0);
        drop(b);
        println!("b dropped before here");
    }
}
