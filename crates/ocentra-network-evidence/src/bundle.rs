use serde::{Deserialize, Serialize};

use crate::cascade::{
    route_network_evidence_cascade, NetworkCascadeNextCheck, NetworkCascadeSignalStrength,
    NetworkCascadeSource, NetworkCascadeSourceKind, NetworkEvidenceCascadeError,
    NetworkEvidenceCascadeInput,
};
use crate::dns::types::NetworkEvidenceGrade;

mod normalize;
mod refs;
mod validation;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkCrossSliceEvidenceSource {
    pub source_kind: NetworkCascadeSourceKind,
    pub signal_strength: NetworkCascadeSignalStrength,
    pub evidence_grade: NetworkEvidenceGrade,
    pub evidence_ref: String,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub policy_action_authority: bool,
    pub adapter_action_authority: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkCrossSliceEvidenceBundleInput {
    pub trigger_ref: String,
    pub sources: Vec<NetworkCrossSliceEvidenceSource>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkCrossSliceEvidenceBundle {
    pub trigger_ref: String,
    pub primary_source: Option<NetworkCascadeSourceKind>,
    pub evidence_refs: Vec<String>,
    pub exact_url_evidence_refs: Vec<String>,
    pub next_checks: Vec<NetworkCascadeNextCheck>,
    pub parent_review_required: bool,
    pub local_ai_review_recommended: bool,
    pub adapter_action_authorized: bool,
    pub policy_action_authority: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub evidence_grade: NetworkEvidenceGrade,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkCrossSliceEvidenceBundleError {
    EmptyTriggerRef,
    EmptyEvidenceRef,
    UnsupportedNetworkExactUrlClaim(NetworkCascadeSourceKind),
    UnsupportedDecryptedPayloadClaim,
    UnsupportedPolicyAuthorityClaim,
    UnsupportedAdapterAuthorityClaim,
}

pub fn build_network_cross_slice_evidence_bundle(
    input: NetworkCrossSliceEvidenceBundleInput,
) -> Result<NetworkCrossSliceEvidenceBundle, NetworkCrossSliceEvidenceBundleError> {
    let NetworkCrossSliceEvidenceBundleInput {
        trigger_ref,
        sources,
    } = input;
    let trigger_ref = normalize::normalize_ref(&trigger_ref)
        .ok_or(NetworkCrossSliceEvidenceBundleError::EmptyTriggerRef)?;
    validation::validate_bundle_sources(&sources)?;

    let cascade_sources = sources
        .iter()
        .map(|source| NetworkCascadeSource {
            source_kind: source.source_kind,
            signal_strength: source.signal_strength,
            evidence_grade: source.evidence_grade,
            source_ref: normalize::normalize_ref(&source.evidence_ref).unwrap_or_default(),
            exact_url_available: source.exact_url_available,
            decrypted_payload_available: source.decrypted_payload_available,
            policy_action_authority: source.policy_action_authority,
        })
        .collect();
    let cascade = route_network_evidence_cascade(NetworkEvidenceCascadeInput {
        sources: cascade_sources,
    })
    .map_err(|error| match error {
        NetworkEvidenceCascadeError::EmptySourceRef => {
            NetworkCrossSliceEvidenceBundleError::EmptyEvidenceRef
        }
        NetworkEvidenceCascadeError::UnsupportedNetworkExactUrlClaim(source_kind) => {
            NetworkCrossSliceEvidenceBundleError::UnsupportedNetworkExactUrlClaim(source_kind)
        }
        NetworkEvidenceCascadeError::UnsupportedDecryptedPayloadClaim => {
            NetworkCrossSliceEvidenceBundleError::UnsupportedDecryptedPayloadClaim
        }
    })?;

    let evidence_refs = refs::unique_evidence_refs(&sources);
    let exact_url_evidence_refs = refs::exact_url_evidence_refs(&sources);
    let exact_url_available = !exact_url_evidence_refs.is_empty();
    let local_ai_review_recommended = cascade
        .next_checks
        .contains(&NetworkCascadeNextCheck::LocalAiReview);

    Ok(NetworkCrossSliceEvidenceBundle {
        trigger_ref,
        primary_source: cascade.primary_source,
        evidence_refs,
        exact_url_evidence_refs,
        next_checks: cascade.next_checks,
        parent_review_required: cascade.parent_review_required,
        local_ai_review_recommended,
        adapter_action_authorized: false,
        policy_action_authority: false,
        exact_url_available,
        decrypted_payload_available: false,
        evidence_grade: cascade.evidence_grade,
    })
}
