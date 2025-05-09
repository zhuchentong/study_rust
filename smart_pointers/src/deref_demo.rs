use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref_demo() {
        let n = 1;
        let b = MyBox(n);

        assert_eq!(n, *b);
        assert_eq!(*b, 1);

        let m = "asdasd";
        let s = String::from(m);
        assert_eq!(&s, &m);
    }
}
