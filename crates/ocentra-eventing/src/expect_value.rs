use std::fmt::Display;

#[doc(hidden)]
pub trait ExpectValue<T> {
    fn expect_value(self, message: &str) -> T;
}

impl<T, E> ExpectValue<T> for Result<T, E>
where
    E: Display,
{
    #[inline]
    fn expect_value(self, _message: &str) -> T {
        match self {
            Ok(value) => value,
            Err(_error) => {
                std::process::abort();
            }
        }
    }
}

impl<T> ExpectValue<T> for Option<T> {
    #[inline]
    fn expect_value(self, _message: &str) -> T {
        match self {
            Some(value) => value,
            None => {
                std::process::abort();
            }
        }
    }
}

