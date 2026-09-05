pub trait StorageCustodyTestValueExt<T> {
    fn assume_ok(self) -> T;
}

impl<T, E: std::fmt::Debug> StorageCustodyTestValueExt<T> for Result<T, E> {
    fn assume_ok(self) -> T {
        match self {
            Ok(value) => value,
            Err(_) => std::process::abort(),
        }
    }
}

impl<T> StorageCustodyTestValueExt<T> for Option<T> {
    fn assume_ok(self) -> T {
        match self {
            Some(value) => value,
            None => std::process::abort(),
        }
    }
}
