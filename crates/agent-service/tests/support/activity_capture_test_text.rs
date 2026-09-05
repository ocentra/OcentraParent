include!("test_text_value.rs");

use std::fmt::{self, Display};

pub(crate) type TestResult = Result<(), TestText>;

pub(crate) fn test_ok<T, E: fmt::Debug>(
    result: Result<T, E>,
    context: impl Display,
) -> Result<T, TestText> {
    result.map_err(|error| TestText::from_display(format!("{context}: {error:?}")))
}
