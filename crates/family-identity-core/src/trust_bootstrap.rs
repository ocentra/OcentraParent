use std::fmt;

use serde::{Deserialize, Serialize};

use crate::household_authority::{HouseholdAuthorityAction, ParentStepUpValidationFailureReason};
use crate::parent_presence::{ParentPresenceChallenge, ParentPresenceVerificationAccepted};

#[path = "trust_bootstrap_authority.rs"]
mod trust_bootstrap_authority;

pub mod current_authority;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default)]
pub(crate) struct TrustBootstrapSealingMarker;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustBootstrapLifecycleIntent {
    #[serde(rename = "seal-parent-device-trust")]
    SealParentDeviceTrust,
}

#[derive(PartialEq, Eq)]
pub struct TrustBootstrapInput {
    pub trust_bootstrap_ref: String,
    pub lifecycle_intent: TrustBootstrapLifecycleIntent,
    pub parent_presence: ParentPresenceVerificationAccepted,
}

/// Opaque, one-use ceremony accepted by both parent-presence and household authority.
///
/// Its receipt is crate-private so downstream runtime callers cannot substitute an
/// action enum, alter the binding, or mint a sealing authorization themselves.
#[derive(PartialEq, Eq)]
pub struct AuthorizedParentDeviceTrustCeremony {
    parent_presence: ParentPresenceVerificationAccepted,
    authority_receipt: SealParentDeviceTrustAuthorityReceipt,
}

#[derive(PartialEq, Eq)]
pub(crate) struct SealParentDeviceTrustAuthorityReceipt {
    parent_presence_receipt_ref: String,
    family_id: String,
    parent_account_id: String,
    device_ref: String,
    action: HouseholdAuthorityAction,
}

#[derive(PartialEq, Eq, Serialize)]
pub struct AwaitingPlatformKeySealingRequest {
    pub trust_bootstrap_ref: String,
    pub device_trust_ref: DeviceTrustRef,
    pub lifecycle_intent: TrustBootstrapLifecycleIntent,
    approved_parent_device_ceremony: ApprovedParentDeviceCeremony,
    pub family_id: String,
    pub parent_account_id: String,
    pub device_ref: String,
    #[serde(skip)]
    pub(crate) sealing_marker: TrustBootstrapSealingMarker,
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedPlatformKeyUnsealingCredential {
    trust_bootstrap_ref: String,
    device_trust_ref: DeviceTrustRef,
    lifecycle_intent: TrustBootstrapLifecycleIntent,
    approved_parent_device_ceremony: ApprovedParentDeviceCeremony,
}

/// Identity binding taken only from the verified parent-presence ceremony.
/// It is intentionally not caller-supplied at the platform sealing boundary.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovedParentDeviceCeremony {
    trust_subject: String,
    device_ref: String,
    device_role: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TrustBootstrapRejection {
    pub parent_step_up_failure_reason: ParentStepUpValidationFailureReason,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DeviceTrustRef(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceTrustRefGenerationFailure {
    EntropyUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrustBootstrapManualRequirementReason {
    AuthorizedChallengeActionUnavailable,
    AuthorityReceiptRequired,
    ChildScopedCeremonyRejected,
    DeviceTrustReferenceGenerationUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TrustBootstrapManualRequirement {
    pub reason: TrustBootstrapManualRequirementReason,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum TrustBootstrapDecision {
    AwaitingPlatformKeySealing(AwaitingPlatformKeySealingRequest),
    Rejected(TrustBootstrapRejection),
    ManualRequired(TrustBootstrapManualRequirement),
}

impl DeviceTrustRef {
    pub fn generate() -> Result<Self, DeviceTrustRefGenerationFailure> {
        let mut random = [0_u8; 32];
        getrandom::fill(&mut random)
            .map_err(|_error| DeviceTrustRefGenerationFailure::EntropyUnavailable)?;
        Ok(Self(encode_hex(&random)))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for DeviceTrustRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DeviceTrustRef")
            .field(&"[redacted]")
            .finish()
    }
}

impl fmt::Debug for TrustBootstrapInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TrustBootstrapInput")
            .field("trust_bootstrap_ref", &"[redacted]")
            .field("lifecycle_intent", &self.lifecycle_intent)
            .field("parent_presence", &"[redacted]")
            .finish()
    }
}

impl fmt::Debug for AwaitingPlatformKeySealingRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AwaitingPlatformKeySealingRequest")
            .field("trust_bootstrap_ref", &"[redacted]")
            .field("device_trust_ref", &"[redacted]")
            .field("lifecycle_intent", &self.lifecycle_intent)
            .field("sealing_marker", &self.sealing_marker)
            .finish()
    }
}

impl fmt::Debug for PersistedPlatformKeyUnsealingCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PersistedPlatformKeyUnsealingCredential")
            .field("trust_bootstrap_ref", &"[redacted]")
            .field("device_trust_ref", &"[redacted]")
            .field("lifecycle_intent", &self.lifecycle_intent)
            .finish()
    }
}

impl AwaitingPlatformKeySealingRequest {
    pub fn consume_for_platform_key_sealing(self) -> PersistedPlatformKeyUnsealingCredential {
        PersistedPlatformKeyUnsealingCredential {
            trust_bootstrap_ref: self.trust_bootstrap_ref,
            device_trust_ref: self.device_trust_ref,
            lifecycle_intent: self.lifecycle_intent,
            approved_parent_device_ceremony: self.approved_parent_device_ceremony,
        }
    }
}

impl PersistedPlatformKeyUnsealingCredential {
    pub fn lifecycle_intent(&self) -> TrustBootstrapLifecycleIntent {
        self.lifecycle_intent
    }

    pub fn device_trust_ref(&self) -> &DeviceTrustRef {
        &self.device_trust_ref
    }

    pub fn trust_bootstrap_ref(&self) -> &str {
        &self.trust_bootstrap_ref
    }

    pub fn approved_parent_device_ceremony(&self) -> &ApprovedParentDeviceCeremony {
        &self.approved_parent_device_ceremony
    }
}

impl ApprovedParentDeviceCeremony {
    pub fn trust_subject(&self) -> &str {
        &self.trust_subject
    }
    pub fn device_ref(&self) -> &str {
        &self.device_ref
    }
    pub fn device_role(&self) -> &str {
        &self.device_role
    }
}

impl fmt::Debug for ApprovedParentDeviceCeremony {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApprovedParentDeviceCeremony")
            .field("trust_subject", &"[redacted]")
            .field("device_ref", &"[redacted]")
            .field("device_role", &"[redacted]")
            .finish()
    }
}

pub fn evaluate_trust_bootstrap(input: TrustBootstrapInput) -> TrustBootstrapDecision {
    trust_bootstrap_authority::evaluate(input, None)
}

/// Runtime entry point from a ceremony accepted by parent presence and household authority.
/// The authority receipt remains opaque; callers cannot fabricate it through this API.
pub fn begin_parent_device_key_sealing(
    trust_bootstrap_ref: String,
    ceremony: AuthorizedParentDeviceTrustCeremony,
) -> TrustBootstrapDecision {
    trust_bootstrap_authority::evaluate(
        TrustBootstrapInput {
            trust_bootstrap_ref,
            lifecycle_intent: TrustBootstrapLifecycleIntent::SealParentDeviceTrust,
            parent_presence: ceremony.parent_presence,
        },
        Some(ceremony.authority_receipt),
    )
}

impl SealParentDeviceTrustAuthorityReceipt {
    pub(crate) fn matches(
        &self,
        parent_presence_receipt_ref: &crate::parent_presence::ParentPresenceReceiptRef,
        challenge: &ParentPresenceChallenge,
    ) -> bool {
        self.parent_presence_receipt_ref == parent_presence_receipt_ref.as_str()
            && self.family_id == challenge.family_id
            && self.parent_account_id == challenge.parent_account_id
            && self.device_ref == challenge.action_device_id
            && self.action == challenge.privileged_action
            && self.action == HouseholdAuthorityAction::SealParentDeviceTrust
    }
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}
