#[doc(hidden)]
pub trait ExpectValue<T> {
    fn expect_value(self, message: &str) -> T;
}

#[doc(hidden)]
pub trait ExpectErrValue<E> {
    fn expect_err_value(self, message: &str) -> E;
}

impl<T, E> ExpectValue<T> for Result<T, E> {
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

impl<T, E> ExpectErrValue<E> for Result<T, E> {
    #[inline]
    fn expect_err_value(self, _message: &str) -> E {
        match self {
            Err(error) => error,
            Ok(_value) => {
                std::process::abort();
            }
        }
    }
}
