pub(crate) trait OptionRequiredExt<T> {
    fn required(self, context: impl std::fmt::Display) -> T;
}

impl<T> OptionRequiredExt<T> for Option<T> {
    fn required(self, context: impl std::fmt::Display) -> T {
        let context = context.to_string();
        self.expect(&context)
    }
}

pub(crate) trait ResultRequiredExt<T, E> {
    fn required(self, context: impl std::fmt::Display) -> T;
}

impl<T, E: std::fmt::Debug> ResultRequiredExt<T, E> for Result<T, E> {
    fn required(self, context: impl std::fmt::Display) -> T {
        let context = context.to_string();
        self.expect(&context)
    }
}
