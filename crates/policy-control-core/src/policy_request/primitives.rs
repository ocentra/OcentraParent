#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use super::{
    PolicyApprovalId, PolicyAssistantPreviewId, PolicyDurationMinutes, PolicyOverrideId,
    PolicyRequestId, PolicyRequestSubmissionKey, PolicyRequestTimestamp,
};

macro_rules! impl_policy_request_text_id {
    ($name:ident, $field:expr) => {
        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                parse_non_empty_text(value, $field).map(Self)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = EventingError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

impl_policy_request_text_id!(PolicyRequestId, policy_control::request::FIELD_REQUEST_ID);
impl_policy_request_text_id!(
    PolicyRequestSubmissionKey,
    policy_control::request::FIELD_SUBMISSION_KEY
);
impl_policy_request_text_id!(PolicyApprovalId, policy_control::request::FIELD_APPROVAL_ID);
impl_policy_request_text_id!(PolicyOverrideId, policy_control::request::FIELD_OVERRIDE_ID);
impl_policy_request_text_id!(
    PolicyAssistantPreviewId,
    policy_control::request::FIELD_ASSISTANT_PREVIEW_ID
);
impl_policy_request_text_id!(
    PolicyRequestTimestamp,
    policy_control::request::FIELD_TIMESTAMP
);

impl PolicyDurationMinutes {
    pub fn new(value: u16) -> Result<Self, EventingError> {
        if value == 0 {
            return Err(EventingError::InvalidValue {
                field: policy_control::request::FIELD_DURATION_MINUTES,
                value: value.to_string(),
            });
        }
        Ok(Self(value))
    }

    pub fn value(self) -> u16 {
        self.0
    }
}

impl TryFrom<u16> for PolicyDurationMinutes {
    type Error = EventingError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PolicyDurationMinutes> for u16 {
    fn from(value: PolicyDurationMinutes) -> Self {
        value.0
    }
}

fn parse_non_empty_text(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, EventingError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(EventingError::EmptyValue { field });
    }

    Ok(value)
}
