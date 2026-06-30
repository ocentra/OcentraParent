pub trait StorageCustodyTestValueExt<T> {
    fn value_or_unreachable(self, context: &str) -> T;
}

impl<T, E> StorageCustodyTestValueExt<T> for Result<T, E> {
    fn value_or_unreachable(self, context: &str) -> T {
        match self {
            Ok(value) => value,
            Err(_) => unreachable!("{context}"),
        }
    }
}

impl<T> StorageCustodyTestValueExt<T> for Option<T> {
    fn value_or_unreachable(self, context: &str) -> T {
        match self {
            Some(value) => value,
            None => unreachable!("{context}"),
        }
    }
}

pub trait StorageCustodyTestErrorExt<E> {
    fn error_or_unreachable(self, context: &str) -> E;
}

impl<T, E> StorageCustodyTestErrorExt<E> for Result<T, E> {
    fn error_or_unreachable(self, context: &str) -> E {
        match self {
            Ok(_) => unreachable!("{context}"),
            Err(error) => error,
        }
    }
}
