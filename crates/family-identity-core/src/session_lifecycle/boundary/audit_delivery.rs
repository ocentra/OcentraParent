#![forbid(unsafe_code)]

//! Raw audit-outbox identifier boundary.

const SESSION_AUDIT_PREFIX: &str = "session-audit-";
const DELIVERY_ATTEMPT_PREFIX: &str = "delivery-attempt-";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionAuditEventId(String);

impl SessionAuditEventId {
    pub(crate) fn generate() -> Result<Self, getrandom::Error> {
        super::storage_values::generate_opaque_identifier(SESSION_AUDIT_PREFIX).map(Self)
    }

    pub(crate) fn parse(value: String) -> Option<Self> {
        super::storage_values::opaque_identifier_is_valid(&value, SESSION_AUDIT_PREFIX)
            .then_some(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct SessionAuditDeliveryAttemptId(String);

impl SessionAuditDeliveryAttemptId {
    pub(crate) fn generate() -> Result<Self, getrandom::Error> {
        super::storage_values::generate_opaque_identifier(DELIVERY_ATTEMPT_PREFIX).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
