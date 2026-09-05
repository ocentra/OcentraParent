use super::{
    text, ProtocolValidationError, ProviderIdentifier, Request, ValidatedRequest,
    MAXIMUM_LEASE_ID_BYTES, MAXIMUM_NONCE_BYTES, MAXIMUM_REQUEST_ID_BYTES, PROTOCOL_VERSION,
};

const RESPONSE_TEXT: [text::ErrorText; 7] = [
    text::PROTOCOL_VERSION_UNSUPPORTED,
    text::IDENTIFIER_INVALID,
    text::RELATIVE_PATH_INVALID,
    text::READ_BOUND_INVALID,
    text::PAYLOAD_BOUND,
    text::PAYLOAD_BOUND_OVERFLOW,
    text::MUTATION_COUNT_INVALID,
];

const RESPONSE_FAILURE: [text::ErrorText; 7] = [
    text::VALIDATION_PROTOCOL_FRAME,
    text::VALIDATION_FAILURE,
    text::VALIDATION_FAILURE,
    text::VALIDATION_SIZE_LIMIT,
    text::VALIDATION_SIZE_LIMIT,
    text::VALIDATION_PROTOCOL_LIMIT,
    text::VALIDATION_PROTOCOL_LIMIT,
];

impl ProtocolValidationError {
    pub(crate) const fn response_text(self) -> text::ErrorText {
        RESPONSE_TEXT[self as usize]
    }

    pub(crate) const fn response_failure(self) -> text::ErrorText {
        RESPONSE_FAILURE[self as usize]
    }
}

impl std::fmt::Display for ProtocolValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self.response_text().message();
        formatter.write_str(&message)
    }
}

impl std::error::Error for ProtocolValidationError {}

impl Request {
    pub(crate) fn validate_frame(&self) -> Result<ValidatedRequest, ProtocolValidationError> {
        if self.protocol_version != PROTOCOL_VERSION {
            return Err(ProtocolValidationError::ProtocolVersion);
        }
        let request_id = ProviderIdentifier::parse(&self.request_id, MAXIMUM_REQUEST_ID_BYTES)?;
        let nonce = ProviderIdentifier::parse(&self.nonce, MAXIMUM_NONCE_BYTES)?;
        let lease_id = self
            .lease_id
            .as_deref()
            .map(|value| ProviderIdentifier::parse(value, MAXIMUM_LEASE_ID_BYTES))
            .transpose()?;
        Ok(ValidatedRequest {
            request_id,
            nonce,
            lease_id,
            operation: self.operation.validate()?,
        })
    }

    pub(crate) fn can_echo(&self) -> bool {
        ProviderIdentifier::is_canonical(&self.request_id, MAXIMUM_REQUEST_ID_BYTES)
            && ProviderIdentifier::is_canonical(&self.nonce, MAXIMUM_NONCE_BYTES)
    }
}
