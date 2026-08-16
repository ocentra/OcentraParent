mod validation;

use serde::{Deserialize, Serialize};

use self::validation::{domains_match, validate_managed_browser_input};

use crate::dns::types::NetworkEvidenceGrade;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManagedBrowserCorrelationState {
    ExactUrlConfirmed,
    NetworkDomainOnly,
    BrowserDomainMismatch,
    MissingManagedBrowserEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManagedBrowserCorrelationBasis {
    ManagedBrowserUrlEvidence,
    NetworkDomainEvidenceOnly,
    MismatchedBrowserDomain,
    NoManagedBrowserEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkManagedBrowserFlowEvidence {
    pub flow_ref: String,
    pub observed_domain: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedBrowserPageEvidence {
    pub browser_ref: String,
    pub tab_ref: String,
    pub page_url: String,
    pub page_domain: String,
    pub source_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedBrowserCorrelationInput {
    pub network_flow: NetworkManagedBrowserFlowEvidence,
    pub managed_browser: Option<ManagedBrowserPageEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedBrowserCorrelation {
    pub state: ManagedBrowserCorrelationState,
    pub basis: ManagedBrowserCorrelationBasis,
    pub network_domain: Option<String>,
    pub browser_domain: Option<String>,
    pub exact_url: Option<String>,
    pub exact_url_source_ref: Option<String>,
    pub exact_url_from_network: bool,
    pub exact_url_from_managed_browser: bool,
    pub decrypted_payload_available: bool,
    pub evidence_refs: Vec<String>,
    pub evidence_grade: NetworkEvidenceGrade,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManagedBrowserCorrelationError {
    EmptyFlowRef,
    EmptyObservedDomain,
    EmptyBrowserRef,
    EmptyTabRef,
    EmptyPageUrl,
    EmptyPageDomain,
    EmptyBrowserSourceRef,
}

pub fn correlate_managed_browser_activity(
    input: ManagedBrowserCorrelationInput,
) -> Result<ManagedBrowserCorrelation, ManagedBrowserCorrelationError> {
    validate_managed_browser_input(&input)?;

    match input.managed_browser {
        Some(browser)
            if domains_match(&input.network_flow.observed_domain, &browser.page_domain) =>
        {
            Ok(exact_url_correlation(input.network_flow, browser))
        }
        Some(browser) => Ok(domain_mismatch_correlation(input.network_flow, browser)),
        None if input.network_flow.observed_domain.is_some() => {
            Ok(network_domain_only_correlation(input.network_flow))
        }
        None => Ok(missing_browser_correlation(input.network_flow.flow_ref)),
    }
}

fn exact_url_correlation(
    network_flow: NetworkManagedBrowserFlowEvidence,
    browser: ManagedBrowserPageEvidence,
) -> ManagedBrowserCorrelation {
    ManagedBrowserCorrelation {
        state: ManagedBrowserCorrelationState::ExactUrlConfirmed,
        basis: ManagedBrowserCorrelationBasis::ManagedBrowserUrlEvidence,
        network_domain: network_flow.observed_domain,
        browser_domain: Some(browser.page_domain),
        exact_url: Some(browser.page_url),
        exact_url_source_ref: Some(browser.source_ref.clone()),
        exact_url_from_network: false,
        exact_url_from_managed_browser: true,
        decrypted_payload_available: false,
        evidence_refs: vec![network_flow.flow_ref, browser.source_ref],
        evidence_grade: NetworkEvidenceGrade::B,
    }
}

fn domain_mismatch_correlation(
    network_flow: NetworkManagedBrowserFlowEvidence,
    browser: ManagedBrowserPageEvidence,
) -> ManagedBrowserCorrelation {
    ManagedBrowserCorrelation {
        state: ManagedBrowserCorrelationState::BrowserDomainMismatch,
        basis: ManagedBrowserCorrelationBasis::MismatchedBrowserDomain,
        network_domain: network_flow.observed_domain,
        browser_domain: Some(browser.page_domain),
        exact_url: None,
        exact_url_source_ref: None,
        exact_url_from_network: false,
        exact_url_from_managed_browser: false,
        decrypted_payload_available: false,
        evidence_refs: vec![network_flow.flow_ref, browser.source_ref],
        evidence_grade: NetworkEvidenceGrade::D,
    }
}

fn network_domain_only_correlation(
    network_flow: NetworkManagedBrowserFlowEvidence,
) -> ManagedBrowserCorrelation {
    ManagedBrowserCorrelation {
        state: ManagedBrowserCorrelationState::NetworkDomainOnly,
        basis: ManagedBrowserCorrelationBasis::NetworkDomainEvidenceOnly,
        network_domain: network_flow.observed_domain,
        browser_domain: None,
        exact_url: None,
        exact_url_source_ref: None,
        exact_url_from_network: false,
        exact_url_from_managed_browser: false,
        decrypted_payload_available: false,
        evidence_refs: vec![network_flow.flow_ref],
        evidence_grade: NetworkEvidenceGrade::C,
    }
}

fn missing_browser_correlation(flow_ref: String) -> ManagedBrowserCorrelation {
    ManagedBrowserCorrelation {
        state: ManagedBrowserCorrelationState::MissingManagedBrowserEvidence,
        basis: ManagedBrowserCorrelationBasis::NoManagedBrowserEvidence,
        network_domain: None,
        browser_domain: None,
        exact_url: None,
        exact_url_source_ref: None,
        exact_url_from_network: false,
        exact_url_from_managed_browser: false,
        decrypted_payload_available: false,
        evidence_refs: vec![flow_ref],
        evidence_grade: NetworkEvidenceGrade::D,
    }
}
