use std::{
    ffi::OsStr,
    fmt::{self, Display},
    ops::Deref,
    path::Path,
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct TestText(pub(crate) String);

impl TestText {
    pub(crate) fn from_display(value: impl Display) -> Self {
        Self(value.to_string())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Display for TestText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for TestText {}

impl AsRef<Path> for TestText {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl AsRef<OsStr> for TestText {
    fn as_ref(&self) -> &OsStr {
        OsStr::new(&self.0)
    }
}

impl Deref for TestText {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl From<&TestText> for TestText {
    fn from(value: &TestText) -> Self {
        value.clone()
    }
}

pub(crate) type TestResult = Result<(), TestText>;

pub(crate) fn test_ok<T, E: std::fmt::Debug>(
    result: Result<T, E>,
    context: impl Display,
) -> Result<T, TestText> {
    result.map_err(|error| TestText::from_display(format!("{context}: {error:?}")))
}

pub(crate) fn test_some<T>(value: Option<T>, context: impl Display) -> Result<T, TestText> {
    value.ok_or_else(|| TestText::from_display(context))
}

pub fn test_err<T, E: std::fmt::Debug>(
    result: Result<T, E>,
    context: impl Display,
) -> Result<E, TestText> {
    match result {
        Ok(_) => Err(TestText::from_display(format!("{context}: expected error"))),
        Err(error) => Ok(error),
    }
}
