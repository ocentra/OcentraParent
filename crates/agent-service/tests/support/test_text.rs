use std::{
    collections::BTreeMap,
    fmt::{self, Display},
    primitive::str as TestStr,
    string::String as TestString,
};

use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub(crate) struct TestText(pub(crate) TestString);

impl TestText {
    pub(crate) fn from_display(value: impl Display) -> Self {
        Self(value.to_string())
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub(crate) fn as_str(&self) -> &TestStr {
        self.0.as_str()
    }
}

impl AsRef<TestStr> for TestText {
    fn as_ref(&self) -> &TestStr {
        self.0.as_str()
    }
}

impl AsRef<[u8]> for TestText {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Display for TestText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for TestText {}

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

pub(crate) fn count_for_display(counts: &BTreeMap<TestText, usize>, key: impl Display) -> usize {
    *counts.get(&TestText::from_display(key)).unwrap_or(&0)
}

pub(crate) fn optional_log_string(payload: &LogFields, field: impl Display) -> Option<TestText> {
    let field_name = field.to_string();
    match payload.get(field_name.as_str()) {
        Some(LogFieldValue::String(value)) => Some(TestText::from_display(value)),
        _ => None,
    }
}

#[cfg(test)]
mod clippy_linkage {
    use super::*;

    #[test]
    fn test_text_helpers_are_linked() {
        let text = TestText::from_display(123);
        assert_eq!(text.as_str(), "123");
        assert_eq!(text.as_bytes(), b"123");

        let ok = test_ok(Ok::<i32, ()>(7), "browser-policy");
        assert_eq!(ok, Ok(7));

        let some = test_some(Some(3), "browser-policy");
        assert_eq!(some, Ok(3));

        let mut counts = BTreeMap::new();
        counts.insert(TestText::from_display("needle"), 4);
        assert_eq!(count_for_display(&counts, "needle"), 4);

        let mut fields = LogFields::new();
        fields.insert(
            "browser".to_string(),
            LogFieldValue::String("policy".to_string()),
        );
        assert_eq!(
            optional_log_string(&fields, "browser"),
            Some(TestText::from_display("policy"))
        );

        let _: TestResult = Ok(());
    }
}
