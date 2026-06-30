pub trait ResultTestExt<T, E> {
    fn value_or_unreachable(self, context: &str) -> T;
    fn error_or_unreachable(self, context: &str) -> E;
}

impl<T, E> ResultTestExt<T, E> for Result<T, E> {
    fn value_or_unreachable(self, context: &str) -> T {
        match self {
            Ok(value) => value,
            Err(_) => unreachable!("{context}"),
        }
    }

    fn error_or_unreachable(self, context: &str) -> E {
        match self {
            Ok(_) => unreachable!("{context}"),
            Err(error) => error,
        }
    }
}

pub trait OptionTestExt<T> {
    fn value_or_unreachable(self, context: &str) -> T;
}

impl<T> OptionTestExt<T> for Option<T> {
    fn value_or_unreachable(self, context: &str) -> T {
        match self {
            Some(value) => value,
            None => unreachable!("{context}"),
        }
    }
}
