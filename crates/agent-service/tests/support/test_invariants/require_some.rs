pub(crate) fn require_some<T>(value: Option<T>, context: impl std::fmt::Display) -> T {
    let _ = context;
    value.unwrap_or_else(|| std::process::abort())
}
