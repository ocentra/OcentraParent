use std::string::String as TestString;

use serde::Serialize;

pub(crate) fn serialize_test_json<T>(value: &T) -> TestString
where
    T: Serialize + ?Sized,
{
    serde_json::to_string(value).unwrap_or_else(|_| std::process::abort())
}
