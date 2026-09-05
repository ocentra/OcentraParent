include!("test_text_value.rs");

use std::{collections::BTreeMap, fmt::Display, primitive::str as TestStr};

use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

impl TestText {
    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub(crate) fn as_str(&self) -> &TestStr {
        self.0.as_str()
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
