//! Rust-owned shape for the desktop-to-phone QR approval bridge.
//!
//! This module owns the wire contract only. It does not verify the issuer
//! signature, authenticate the approving phone, consume the nonce, or claim
//! that a transport or platform ceremony exists.

use serde::{Deserialize, Serialize};

pub const PHONE_QR_APPROVAL_SCHEMA_VERSION: &str = "phone-qr-approval-bridge-v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PhoneQrApprovalResult {
    Approved,
    Rejected,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PhoneQrApprovalReplayState {
    Fresh,
    Consumed,
    ReplayRejected,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhoneQrApprovalChallenge {
    pub schema_version: String,
    pub challenge_id: String,
    pub action_ref: String,
    pub household_ref: String,
    pub parent_account_ref: String,
    pub desktop_device_ref: String,
    pub target_ref: String,
    pub issued_at: String,
    pub expires_at: String,
    pub nonce_or_challenge_ref: String,
    pub audit_ref: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhoneQrApprovalResponse {
    pub schema_version: String,
    pub approval_id: String,
    pub challenge_id: String,
    pub action_ref: String,
    pub household_ref: String,
    pub parent_account_ref: String,
    pub approving_device_ref: String,
    pub desktop_device_ref: String,
    pub target_ref: String,
    pub issued_at: String,
    pub approved_at: String,
    pub expires_at: String,
    pub nonce_or_challenge_ref: String,
    pub audit_ref: String,
    pub approval_result: PhoneQrApprovalResult,
    pub replay_state: PhoneQrApprovalReplayState,
    pub issuer: String,
    pub audience: String,
    pub key_id: String,
    pub signature: String,
}
