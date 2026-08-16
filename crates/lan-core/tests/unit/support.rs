pub trait ResultTestExt<T, E> {
    fn value_or_unreachable(self) -> T;
    fn error_or_unreachable(self) -> E;
}

impl<T, E> ResultTestExt<T, E> for Result<T, E> {
    fn value_or_unreachable(self) -> T {
        match self {
            Ok(value) => value,
            Err(_) => std::process::abort(),
        }
    }

    fn error_or_unreachable(self) -> E {
        match self {
            Ok(_) => std::process::abort(),
            Err(error) => error,
        }
    }
}

pub trait OptionTestExt<T> {
    fn value_or_unreachable(self) -> T;
}

impl<T> OptionTestExt<T> for Option<T> {
    fn value_or_unreachable(self) -> T {
        match self {
            Some(value) => value,
            None => std::process::abort(),
        }
    }
}
