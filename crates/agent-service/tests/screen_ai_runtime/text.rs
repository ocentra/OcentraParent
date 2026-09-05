#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TestText(pub(crate) String);

impl TestText {
    pub(crate) fn from_display(value: impl std::fmt::Display) -> Self {
        Self(value.to_string())
    }

    pub(crate) fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for TestText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}
