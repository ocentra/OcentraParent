#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub(crate) struct TestText(pub(crate) std::string::String);

impl TestText {
    pub(crate) fn from_display(value: impl std::fmt::Display) -> Self {
        Self(value.to_string())
    }
}

impl AsRef<str> for TestText {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<[u8]> for TestText {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl std::fmt::Display for TestText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

impl std::error::Error for TestText {}
