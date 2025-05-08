//! A library for profiling.
//!
//! This library provides functions for profiling
//!  .
//!
pub use self::test1::add;
pub use self::test2::multiply;

pub mod test1 {
    /// Adds two numbers together.
    ///
    /// # Examples
    ///     
    /// ```
    /// let result = profile::add(1, 2);
    /// assert_eq!(result, 3);
    ///     
    /// ``````
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

pub mod test2 {
    /// Multiplies two numbers together.
    ///
    /// # Examples
    ///     
    /// ```
    /// let result = profile::multiply(2, 3);
    /// assert_eq!(result, 6);
    ///     
    /// ``````
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}
