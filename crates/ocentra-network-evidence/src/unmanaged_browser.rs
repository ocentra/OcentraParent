mod validation;

use serde::{Deserialize, Serialize};

use self::validation::validate_unmanaged_browser_input;

use crate::dns::types::NetworkEvidenceGrade;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnmanagedBrowserProcessKind {
    KnownBrowser,
    PortableBrowser,
    BrowserLikeProcess,
    ManagedOcentraBrowser,
    NonBrowserProcess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnmanagedBrowserCorrelationState {
    ProcessOnlyBypassEvidence,
    ProcessOnlyBypassCandidate,
    ManagedBrowserBoundary,
    NoUnmanagedBrowserEvidence,
    AdapterUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnmanagedBrowserCorrelationBasis {
    KnownBrowserProcess,
    PortableBrowserProcess,
    BrowserLikeProcessName,
    ManagedBrowserProcess,
    MissingBrowserLikeProcess,
    AdapterUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnmanagedBrowserCorrelationInput {
    pub observation_ref: String,
    pub adapter_available: bool,
    pub process_id: Option<u32>,
    pub process_name: Option<String>,
    pub redacted_executable_path_ref: Option<String>,
    pub signature_ref: Option<String>,
    pub hash_ref: Option<String>,
    pub browser_family: Option<String>,
    pub process_kind: Option<UnmanagedBrowserProcessKind>,
    pub confidence: Option<u8>,
    pub possible_bypass_reason_ref: Option<String>,
    pub exact_url_claimed: bool,
    pub active_tab_claimed: bool,
    pub page_title_claimed: bool,
    pub page_content_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub policy_action_authority: bool,
    pub adapter_action_authorized: bool,
    pub enforcement_command_published: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnmanagedBrowserCorrelation {
    pub state: UnmanagedBrowserCorrelationState,
    pub basis: UnmanagedBrowserCorrelationBasis,
    pub process_id: Option<u32>,
    pub process_name: Option<String>,
    pub redacted_executable_path_ref: Option<String>,
    pub signature_ref: Option<String>,
    pub hash_ref: Option<String>,
    pub browser_family: Option<String>,
    pub confidence: Option<u8>,
    pub possible_bypass: bool,
    pub possible_bypass_reason_ref: Option<String>,
    pub exact_url_available: bool,
    pub active_tab_available: bool,
    pub page_title_available: bool,
    pub page_content_available: bool,
    pub decrypted_payload_available: bool,
    pub policy_action_authority: bool,
    pub adapter_action_authorized: bool,
    pub enforcement_command_published: bool,
    pub evidence_refs: Vec<String>,
    pub evidence_grade: NetworkEvidenceGrade,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnmanagedBrowserCorrelationError {
    EmptyObservationRef,
    EmptyProcessName,
    EmptyRedactedExecutablePathRef,
    EmptySignatureRef,
    EmptyHashRef,
    EmptyBrowserFamily,
    EmptyPossibleBypassReasonRef,
    InvalidConfidence(u8),
    UnsupportedExactUrlClaim,
    UnsupportedActiveTabClaim,
    UnsupportedPageTitleClaim,
    UnsupportedPageContentClaim,
    UnsupportedDecryptedPayloadClaim,
    UnsupportedPolicyAuthorityClaim,
    UnsupportedAdapterAuthorityClaim,
    UnsupportedEnforcementCommandClaim,
}

pub fn correlate_unmanaged_browser_activity(
    input: UnmanagedBrowserCorrelationInput,
) -> Result<UnmanagedBrowserCorrelation, UnmanagedBrowserCorrelationError> {
    validate_unmanaged_browser_input(&input)?;

    if !input.adapter_available {
        return Ok(base_unmanaged_browser_correlation(
            UnmanagedBrowserCorrelationState::AdapterUnavailable,
            UnmanagedBrowserCorrelationBasis::AdapterUnavailable,
            false,
            NetworkEvidenceGrade::D,
            input,
        ));
    }

    match input.process_kind {
        Some(UnmanagedBrowserProcessKind::KnownBrowser) => Ok(base_unmanaged_browser_correlation(
            UnmanagedBrowserCorrelationState::ProcessOnlyBypassEvidence,
            UnmanagedBrowserCorrelationBasis::KnownBrowserProcess,
            true,
            NetworkEvidenceGrade::C,
            input,
        )),
        Some(UnmanagedBrowserProcessKind::PortableBrowser) => {
            Ok(base_unmanaged_browser_correlation(
                UnmanagedBrowserCorrelationState::ProcessOnlyBypassEvidence,
                UnmanagedBrowserCorrelationBasis::PortableBrowserProcess,
                true,
                NetworkEvidenceGrade::C,
                input,
            ))
        }
        Some(UnmanagedBrowserProcessKind::BrowserLikeProcess) => {
            Ok(base_unmanaged_browser_correlation(
                UnmanagedBrowserCorrelationState::ProcessOnlyBypassCandidate,
                UnmanagedBrowserCorrelationBasis::BrowserLikeProcessName,
                true,
                NetworkEvidenceGrade::D,
                input,
            ))
        }
        Some(UnmanagedBrowserProcessKind::ManagedOcentraBrowser) => {
            Ok(base_unmanaged_browser_correlation(
                UnmanagedBrowserCorrelationState::ManagedBrowserBoundary,
                UnmanagedBrowserCorrelationBasis::ManagedBrowserProcess,
                false,
                NetworkEvidenceGrade::D,
                input,
            ))
        }
        Some(UnmanagedBrowserProcessKind::NonBrowserProcess) | None => {
            Ok(base_unmanaged_browser_correlation(
                UnmanagedBrowserCorrelationState::NoUnmanagedBrowserEvidence,
                UnmanagedBrowserCorrelationBasis::MissingBrowserLikeProcess,
                false,
                NetworkEvidenceGrade::D,
                input,
            ))
        }
    }
}

fn base_unmanaged_browser_correlation(
    state: UnmanagedBrowserCorrelationState,
    basis: UnmanagedBrowserCorrelationBasis,
    possible_bypass: bool,
    evidence_grade: NetworkEvidenceGrade,
    input: UnmanagedBrowserCorrelationInput,
) -> UnmanagedBrowserCorrelation {
    UnmanagedBrowserCorrelation {
        state,
        basis,
        process_id: input.process_id,
        process_name: trimmed_option(input.process_name),
        redacted_executable_path_ref: trimmed_option(input.redacted_executable_path_ref),
        signature_ref: trimmed_option(input.signature_ref),
        hash_ref: trimmed_option(input.hash_ref),
        browser_family: trimmed_option(input.browser_family),
        confidence: input.confidence,
        possible_bypass,
        possible_bypass_reason_ref: trimmed_option(input.possible_bypass_reason_ref),
        exact_url_available: false,
        active_tab_available: false,
        page_title_available: false,
        page_content_available: false,
        decrypted_payload_available: false,
        policy_action_authority: false,
        adapter_action_authorized: false,
        enforcement_command_published: false,
        evidence_refs: vec![input.observation_ref],
        evidence_grade,
    }
}

fn trimmed_option(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}
