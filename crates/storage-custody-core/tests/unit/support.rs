pub trait StorageCustodyTestValueExt<T> {
    fn assume_ok(self) -> T;
}

impl<T, E: std::fmt::Debug> StorageCustodyTestValueExt<T> for Result<T, E> {
    fn assume_ok(self) -> T {
        self.unwrap()
    }
}

impl<T> StorageCustodyTestValueExt<T> for Option<T> {
    fn assume_ok(self) -> T {
        self.unwrap()
    }
}

pub trait StorageCustodyTestErrorExt<E> {
    fn assume_err(self) -> E;
}

impl<T, E> StorageCustodyTestErrorExt<E> for Result<T, E> {
    fn assume_err(self) -> E {
        self.err().unwrap()
    }
}
