use serde::Serialize;
use serde_json::Value;

use super::{
    text, ProtocolValidationError, ProviderIdentifier, ProviderPayload, ProviderRelativePath,
    ReadMaximum, MAXIMUM_READ_BYTES, MAXIMUM_RELATIVE_PATH_BYTES,
};

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub(crate) struct ResponseResult(Value);

impl From<Value> for ResponseResult {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

impl ResponseResult {
    pub(crate) fn into_value(self) -> Value {
        self.0
    }
}

impl ProviderIdentifier {
    pub(super) fn parse(
        value: &str,
        maximum_bytes: usize,
    ) -> Result<Self, ProtocolValidationError> {
        if !Self::is_canonical(value, maximum_bytes) {
            return Err(ProtocolValidationError::Identifier);
        }
        Ok(Self(value.to_owned()))
    }

    pub(crate) fn is_canonical(value: &str, maximum_bytes: usize) -> bool {
        !value.is_empty()
            && value.len() <= maximum_bytes
            && value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    }

    pub(crate) fn into_text(self) -> String {
        self.0
    }

    /// CLONE-JUSTIFICATION: response authority echoes are emitted after the
    /// validated request remains borrowed by the operation dispatcher.
    pub(crate) fn text(&self) -> String {
        self.0.clone()
    }

    pub(crate) fn generated(value: String) -> Self {
        Self(value)
    }
}

impl ProviderRelativePath {
    pub(super) fn parse(value: &str, allow_empty: bool) -> Result<Self, ProtocolValidationError> {
        let dot = text::TextId::Dot.text();
        let dot_dot = text::TextId::DotDot.text();
        if (!allow_empty && value.is_empty())
            || value.len() > MAXIMUM_RELATIVE_PATH_BYTES
            || value.contains('\0')
            || value.contains(':')
            || value.starts_with('/')
            || value.contains('\\')
            || (!value.is_empty()
                && value
                    .split('/')
                    .any(|part| part.is_empty() || part == dot || part == dot_dot))
        {
            return Err(ProtocolValidationError::RelativePath);
        }
        Ok(Self(value.to_owned()))
    }

    /// CLONE-JUSTIFICATION: the native owner borrows an owned path string
    /// while the validated request retains the authoritative path value.
    pub(crate) fn text(&self) -> String {
        self.0.clone()
    }

    pub(crate) fn transaction_marker() -> Self {
        Self(text::TextId::ReceiptTransaction.text())
    }
}

impl ProviderPayload {
    pub(super) fn parse(
        value: &str,
        maximum_decoded_bytes: usize,
        allow_empty: bool,
    ) -> Result<Self, ProtocolValidationError> {
        let maximum_encoded_bytes = maximum_decoded_bytes
            .checked_add(2)
            .and_then(|bytes| bytes.checked_div(3))
            .and_then(|groups| groups.checked_mul(4))
            .ok_or(ProtocolValidationError::PayloadBoundOverflow)?;
        if (!allow_empty && value.is_empty()) || value.len() > maximum_encoded_bytes {
            return Err(ProtocolValidationError::PayloadBound);
        }
        Ok(Self(value.to_owned()))
    }

    /// CLONE-JUSTIFICATION: base64 decoding borrows an owned encoded string
    /// while the validated operation retains its wire payload.
    pub(crate) fn text(&self) -> String {
        self.0.clone()
    }
}

impl ReadMaximum {
    pub(super) fn parse(value: u64) -> Result<Self, ProtocolValidationError> {
        if value == 0 || value > MAXIMUM_READ_BYTES {
            return Err(ProtocolValidationError::ReadBound);
        }
        Ok(Self(value))
    }

    pub(crate) const fn value(self) -> u64 {
        self.0
    }
}
