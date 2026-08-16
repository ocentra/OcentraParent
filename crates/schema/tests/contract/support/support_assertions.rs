use super::AssertionContext;

pub(super) fn option_or_unreachable<T>(value: Option<T>, context: AssertionContext<'_>) -> T {
    assert_eq!(value.as_ref().map(|_| ()), Some(()), "{}", context.0);
    match value {
        Some(value) => value,
        None => std::process::abort(),
    }
}

pub(super) fn result_or_unreachable<T, E: std::fmt::Debug>(
    value: Result<T, E>,
    context: AssertionContext<'_>,
) -> T {
    assert_eq!(
        value.as_ref().map(|_| ()).map_err(|_error| ()),
        Ok(()),
        "{}: {:?}",
        context.0,
        value.as_ref().err()
    );
    match value {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    }
}

pub(super) fn error_or_unreachable<T: std::fmt::Debug, E: std::fmt::Debug>(
    value: Result<T, E>,
    context: AssertionContext<'_>,
) -> E {
    assert_eq!(
        value.as_ref().map(|_| ()).map_err(|_error| ()),
        Err(()),
        "{}: {:?}",
        context.0,
        value.as_ref().ok()
    );
    match value {
        Ok(_) => std::process::abort(),
        Err(error) => error,
    }
}
