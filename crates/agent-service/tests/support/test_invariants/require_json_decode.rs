use serde::de::DeserializeOwned;

pub(crate) fn require_json_decode<T>(text: impl AsRef<[u8]>, context: impl std::fmt::Display) -> T
where
    T: DeserializeOwned,
{
    let _ = context;
    serde_json::from_slice(text.as_ref()).unwrap_or_else(|_| std::process::abort())
}
