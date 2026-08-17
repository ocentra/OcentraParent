use chrono::{DateTime, TimeDelta};
use ocentra_schema::parent_step_up_receipt::{
    ParentStepUpAuthorityReceipt, PARENT_STEP_UP_RECEIPT_SCHEMA_VERSION,
};

use crate::household_authority::{HouseholdAuthorityAction, ParentStepUpAssertionSnapshot};
use crate::parent_presence::ParentPresenceObservedAt;

const MAX_PARENT_STEP_UP_RECEIPT_LIFETIME_SECONDS: i64 = 5 * 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentStepUpAuthorityFailure {
    InvalidReceiptShape,
    Required,
    Expired,
    WrongHousehold,
    WrongAccount,
    WrongAction,
    WrongDevice,
    WrongTarget,
    ReplayRejected,
    SignatureUnverified,
    AuthorityUnavailable,
    TimestampInvalid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentStepUpAuthorityRequest {
    pub issuer: String,
    pub audience: String,
    pub family_id: String,
    pub parent_account_id: String,
    pub action_device_id: String,
    pub action_device_child_profile_id: Option<String>,
    pub target_child_profile_id: Option<String>,
    pub action: HouseholdAuthorityAction,
    pub expected_nonce: String,
    pub observed_at: String,
}

pub trait ParentStepUpAuthorityVerifier {
    fn verify_and_consume(
        &mut self,
        receipt: &ParentStepUpAuthorityReceipt,
        request: &ParentStepUpAuthorityRequest,
    ) -> Result<ParentStepUpAssertionSnapshot, ParentStepUpAuthorityFailure>;
}

#[derive(Debug, Default)]
pub struct UnavailableParentStepUpAuthorityVerifier;

impl ParentStepUpAuthorityVerifier for UnavailableParentStepUpAuthorityVerifier {
    fn verify_and_consume(
        &mut self,
        _receipt: &ParentStepUpAuthorityReceipt,
        _request: &ParentStepUpAuthorityRequest,
    ) -> Result<ParentStepUpAssertionSnapshot, ParentStepUpAuthorityFailure> {
        Err(ParentStepUpAuthorityFailure::AuthorityUnavailable)
    }
}

pub fn verify_parent_step_up_receipt(
    _verifier: &mut impl ParentStepUpAuthorityVerifier,
    receipt: &ParentStepUpAuthorityReceipt,
    request: &ParentStepUpAuthorityRequest,
) -> Result<ParentStepUpAssertionSnapshot, ParentStepUpAuthorityFailure> {
    validate_receipt_shape(receipt, request)?;
    // A receipt shape is not parent authority.  Until the family-owned
    // platform/passkey adapter owns signature verification and one-time nonce
    // consumption, accepting a caller-provided verifier would let any crate
    // mint an assertion snapshot by implementing this trait.  Keep the
    // boundary fail-closed and preserve the explicit unavailable outcome.
    Err(ParentStepUpAuthorityFailure::AuthorityUnavailable)
}

fn validate_receipt_shape(
    receipt: &ParentStepUpAuthorityReceipt,
    request: &ParentStepUpAuthorityRequest,
) -> Result<(), ParentStepUpAuthorityFailure> {
    let observed_at = ParentPresenceObservedAt::from_canonical_utc(&request.observed_at)
        .map_err(|_error| ParentStepUpAuthorityFailure::TimestampInvalid)?;
    let issued_at = ParentPresenceObservedAt::from_canonical_utc(&receipt.issued_at)
        .map_err(|_error| ParentStepUpAuthorityFailure::TimestampInvalid)?;
    let expires_at = ParentPresenceObservedAt::from_canonical_utc(&receipt.expires_at)
        .map_err(|_error| ParentStepUpAuthorityFailure::TimestampInvalid)?;

    let shape_checks = [
        receipt.schema_version == PARENT_STEP_UP_RECEIPT_SCHEMA_VERSION,
        !receipt.receipt_id.is_empty(),
        !receipt.key_id.is_empty(),
        !receipt.issuer.is_empty(),
        !receipt.audience.is_empty(),
        !receipt.family_id.is_empty(),
        !receipt.parent_account_id.is_empty(),
        !receipt.action_device_id.is_empty(),
        !receipt.nonce.is_empty(),
        !receipt.signature.is_empty(),
        receipt.issuer == request.issuer,
        receipt.audience == request.audience,
        receipt.family_id == request.family_id,
        receipt.parent_account_id == request.parent_account_id,
        receipt.action_device_id == request.action_device_id,
        receipt.action_device_child_profile_id == request.action_device_child_profile_id,
        receipt.target_child_profile_id == request.target_child_profile_id,
        receipt.nonce == request.expected_nonce,
    ];
    if shape_checks.iter().any(|valid| !valid) {
        return Err(ParentStepUpAuthorityFailure::InvalidReceiptShape);
    }

    let action: HouseholdAuthorityAction =
        serde_json::from_value(serde_json::Value::String(receipt.action.clone()))
            .map_err(|_error| ParentStepUpAuthorityFailure::WrongAction)?;
    if action != request.action {
        return Err(ParentStepUpAuthorityFailure::WrongAction);
    }

    if !observed_at.is_before(&expires_at) {
        return Err(ParentStepUpAuthorityFailure::Expired);
    }
    if issued_at.is_after(&observed_at) {
        return Err(ParentStepUpAuthorityFailure::InvalidReceiptShape);
    }
    let issued_at = DateTime::parse_from_rfc3339(&receipt.issued_at)
        .map_err(|_error| ParentStepUpAuthorityFailure::TimestampInvalid)?;
    let expires_at = DateTime::parse_from_rfc3339(&receipt.expires_at)
        .map_err(|_error| ParentStepUpAuthorityFailure::TimestampInvalid)?;
    if expires_at - issued_at > TimeDelta::seconds(MAX_PARENT_STEP_UP_RECEIPT_LIFETIME_SECONDS) {
        return Err(ParentStepUpAuthorityFailure::InvalidReceiptShape);
    }
    Ok(())
}
