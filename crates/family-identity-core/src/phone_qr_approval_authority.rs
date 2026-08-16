use chrono::{DateTime, TimeDelta};
use ocentra_schema::phone_qr_approval::{
    PhoneQrApprovalChallenge, PhoneQrApprovalReplayState, PhoneQrApprovalResponse,
    PhoneQrApprovalResult, PHONE_QR_APPROVAL_SCHEMA_VERSION,
};

use crate::parent_presence::ParentPresenceObservedAt;

const MAX_PHONE_QR_APPROVAL_LIFETIME_SECONDS: i64 = 5 * 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhoneQrApprovalAuthorityFailure {
    InvalidChallengeShape,
    InvalidResponseShape,
    Required,
    Expired,
    WrongAction,
    WrongHousehold,
    WrongParentAccount,
    WrongApprovingDevice,
    WrongDesktopDevice,
    WrongTarget,
    ReplayRejected,
    SignatureUnverified,
    AuthorityUnavailable,
    TimestampInvalid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhoneQrApprovalRequest {
    pub issuer: String,
    pub audience: String,
    pub action_ref: String,
    pub household_ref: String,
    pub parent_account_ref: String,
    /// Trusted desktop/session context; never derive this from the response.
    pub expected_approving_device_ref: String,
    pub desktop_device_ref: String,
    pub target_ref: String,
    pub expected_nonce_or_challenge_ref: String,
    pub observed_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhoneQrApprovalAssertionSnapshot {
    pub challenge_id: String,
    pub approval_id: String,
    pub approving_device_ref: String,
    pub action_ref: String,
    pub household_ref: String,
    pub parent_account_ref: String,
    pub desktop_device_ref: String,
    pub target_ref: String,
    pub nonce_or_challenge_ref: String,
    pub audit_ref: String,
    pub expires_at: String,
}

pub trait PhoneQrApprovalAuthorityVerifier {
    fn verify_and_consume(
        &mut self,
        challenge: &PhoneQrApprovalChallenge,
        response: &PhoneQrApprovalResponse,
        request: &PhoneQrApprovalRequest,
    ) -> Result<PhoneQrApprovalAssertionSnapshot, PhoneQrApprovalAuthorityFailure>;
}

#[derive(Debug, Default)]
pub struct UnavailablePhoneQrApprovalAuthorityVerifier;

impl PhoneQrApprovalAuthorityVerifier for UnavailablePhoneQrApprovalAuthorityVerifier {
    fn verify_and_consume(
        &mut self,
        _challenge: &PhoneQrApprovalChallenge,
        _response: &PhoneQrApprovalResponse,
        _request: &PhoneQrApprovalRequest,
    ) -> Result<PhoneQrApprovalAssertionSnapshot, PhoneQrApprovalAuthorityFailure> {
        Err(PhoneQrApprovalAuthorityFailure::AuthorityUnavailable)
    }
}

pub fn verify_phone_qr_approval(
    verifier: &mut impl PhoneQrApprovalAuthorityVerifier,
    challenge: &PhoneQrApprovalChallenge,
    response: &PhoneQrApprovalResponse,
    request: &PhoneQrApprovalRequest,
) -> Result<PhoneQrApprovalAssertionSnapshot, PhoneQrApprovalAuthorityFailure> {
    validate_challenge_shape(challenge, request)?;
    validate_response_shape(challenge, response, request)?;
    verifier.verify_and_consume(challenge, response, request)
}

fn validate_challenge_shape(
    challenge: &PhoneQrApprovalChallenge,
    request: &PhoneQrApprovalRequest,
) -> Result<(), PhoneQrApprovalAuthorityFailure> {
    let observed_at = parse_timestamp(&request.observed_at)?;
    let issued_at = parse_timestamp(&challenge.issued_at)?;
    let expires_at = parse_timestamp(&challenge.expires_at)?;
    if challenge.schema_version != PHONE_QR_APPROVAL_SCHEMA_VERSION
        || challenge.challenge_id.is_empty()
        || challenge.action_ref.is_empty()
        || challenge.household_ref.is_empty()
        || challenge.parent_account_ref.is_empty()
        || challenge.desktop_device_ref.is_empty()
        || challenge.target_ref.is_empty()
        || challenge.nonce_or_challenge_ref.is_empty()
        || challenge.audit_ref.is_empty()
        || challenge.action_ref != request.action_ref
        || challenge.household_ref != request.household_ref
        || challenge.parent_account_ref != request.parent_account_ref
        || challenge.desktop_device_ref != request.desktop_device_ref
        || challenge.target_ref != request.target_ref
        || challenge.nonce_or_challenge_ref != request.expected_nonce_or_challenge_ref
    {
        return Err(PhoneQrApprovalAuthorityFailure::InvalidChallengeShape);
    }
    if issued_at >= expires_at || issued_at > observed_at {
        return Err(PhoneQrApprovalAuthorityFailure::Expired);
    }
    if expires_at - issued_at > TimeDelta::seconds(MAX_PHONE_QR_APPROVAL_LIFETIME_SECONDS) {
        return Err(PhoneQrApprovalAuthorityFailure::InvalidChallengeShape);
    }
    if observed_at >= expires_at {
        return Err(PhoneQrApprovalAuthorityFailure::Expired);
    }
    Ok(())
}

fn validate_response_shape(
    challenge: &PhoneQrApprovalChallenge,
    response: &PhoneQrApprovalResponse,
    request: &PhoneQrApprovalRequest,
) -> Result<(), PhoneQrApprovalAuthorityFailure> {
    let observed_at = parse_timestamp(&request.observed_at)?;
    let challenge_issued_at = parse_timestamp(&challenge.issued_at)?;
    let issued_at = parse_timestamp(&response.issued_at)?;
    let approved_at = parse_timestamp(&response.approved_at)?;
    let expires_at = parse_timestamp(&response.expires_at)?;
    if response.schema_version != PHONE_QR_APPROVAL_SCHEMA_VERSION
        || response.approval_id.is_empty()
        || response.challenge_id != challenge.challenge_id
        || response.action_ref != challenge.action_ref
        || response.household_ref != challenge.household_ref
        || response.parent_account_ref != challenge.parent_account_ref
        || response.approving_device_ref.is_empty()
        || request.expected_approving_device_ref.is_empty()
        || response.approving_device_ref != request.expected_approving_device_ref
        || response.desktop_device_ref != challenge.desktop_device_ref
        || response.target_ref != challenge.target_ref
        || response.nonce_or_challenge_ref != challenge.nonce_or_challenge_ref
        || response.audit_ref != challenge.audit_ref
        || response.expires_at != challenge.expires_at
        || response.issuer != request.issuer
        || response.audience != request.audience
        || response.key_id.is_empty()
        || response.signature.is_empty()
    {
        return Err(PhoneQrApprovalAuthorityFailure::InvalidResponseShape);
    }
    if response.approval_result != PhoneQrApprovalResult::Approved {
        return Err(PhoneQrApprovalAuthorityFailure::Required);
    }
    if response.replay_state != PhoneQrApprovalReplayState::Fresh {
        return Err(PhoneQrApprovalAuthorityFailure::ReplayRejected);
    }
    if issued_at < challenge_issued_at
        || approved_at < challenge_issued_at
        || issued_at > approved_at
        || approved_at >= expires_at
        || approved_at > observed_at
    {
        return Err(PhoneQrApprovalAuthorityFailure::Expired);
    }
    Ok(())
}

fn parse_timestamp(
    value: &str,
) -> Result<DateTime<chrono::FixedOffset>, PhoneQrApprovalAuthorityFailure> {
    ParentPresenceObservedAt::from_canonical_utc(value)
        .map_err(|_error| PhoneQrApprovalAuthorityFailure::TimestampInvalid)?;
    DateTime::parse_from_rfc3339(value)
        .map_err(|_error| PhoneQrApprovalAuthorityFailure::TimestampInvalid)
}
