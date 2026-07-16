pub(crate) trait ResultRequiredExt<T, E> {
    fn required(self, context: impl std::fmt::Display) -> T;
}

impl<T, E: std::fmt::Debug> ResultRequiredExt<T, E> for Result<T, E> {
    fn required(self, context: impl std::fmt::Display) -> T {
        let context = context.to_string();
        let _ = context;
        self.unwrap_or_else(|_| std::process::abort())
    }
}
