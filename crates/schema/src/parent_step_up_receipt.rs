//! Rust-owned contract for a remotely issued parent step-up receipt.
//!
//! The receipt is an input from the account/RP authority.  This crate owns its
//! encoded shape only; it does not verify WebAuthn, signatures, or consume a
//! nonce.  Those operations belong to the authority adapter and must remain
//! fail-closed until a real provider is wired.

use serde::{Deserialize, Serialize};

pub const PARENT_STEP_UP_RECEIPT_SCHEMA_VERSION: &str = "parent-step-up-receipt-v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentStepUpAuthorityReceipt {
    pub schema_version: String,
    pub receipt_id: String,
    pub issuer: String,
    pub audience: String,
    pub key_id: String,
    pub family_id: String,
    pub parent_account_id: String,
    pub action_device_id: String,
    pub action_device_child_profile_id: Option<String>,
    pub target_child_profile_id: Option<String>,
    pub action: String,
    pub nonce: String,
    pub issued_at: String,
    pub expires_at: String,
    pub signature: String,
}
