use std::fmt::Debug;

pub(crate) fn require_ok<T, E>(result: Result<T, E>, context: impl std::fmt::Display) -> T
where
    E: Debug,
{
    let _ = context;
    result.unwrap_or_else(|_| std::process::abort())
}
