use ocentra_parent_agent_protocol::activity::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::browser::{
    BrowserActiveProofSource, BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel,
    BrowserCustodyLabel, BrowserFamily,
};
use ocentra_parent_agent_protocol::browser_managed::BrowserQueryVisibilityLabel;
use ocentra_parent_agent_protocol::constants;

use crate::browser_bridge_fields::{
    base_browser_fields, insert_optional_text, normalized_browser_url,
};
use crate::browser_bridge_ids::{
    browser_event_id, browser_evidence_id, browser_subject_id, browser_tab_id,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserBridgeTargetObservation {
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub managed_browser_session_id: String,
    pub profile_id: String,
    pub process_id: u32,
    pub target_id: String,
    pub tab_id: Option<String>,
    pub window_id: Option<String>,
    pub active_state: BrowserActiveTabState,
    pub active_proof_source: BrowserActiveProofSource,
    pub url: String,
    pub title: Option<String>,
    pub capability_status: BrowserCapabilityStatus,
    pub degraded_reason: Option<String>,
    pub custody_label: BrowserCustodyLabel,
    pub query_visibility: BrowserQueryVisibilityLabel,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrowserBridgeEventError {
    InvalidTargetId,
    InvalidUrl,
}

pub fn browser_tab_observation_event(
    observation: BrowserBridgeTargetObservation,
    observed_at: &str,
    fresh_until: &str,
    sequence_index: usize,
) -> Result<ActivityEvent, BrowserBridgeEventError> {
    if observation.target_id.is_empty() {
        return Err(BrowserBridgeEventError::InvalidTargetId);
    }
    let normalized =
        normalized_browser_url(&observation.url).ok_or(BrowserBridgeEventError::InvalidUrl)?;
    let evidence_id = browser_evidence_id(&observation, observed_at, sequence_index);
    let mut fields = base_browser_fields(&observation, &evidence_id, fresh_until, &normalized);
    let tab_id = observation
        .tab_id
        .clone()
        .or_else(|| Some(browser_tab_id(&observation.target_id)));
    insert_optional_text(&mut fields, constants::field::TAB_ID, &tab_id);
    insert_optional_text(
        &mut fields,
        constants::field::WINDOW_ID,
        &observation.window_id,
    );
    insert_optional_text(&mut fields, constants::field::TITLE, &observation.title);

    Ok(ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: browser_event_id(&observation, observed_at, sequence_index),
        observed_at: observed_at.to_string(),
        source: ActivitySource {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: std::env::consts::OS.to_string(),
            observer: ActivityObserver::ManagedBrowserBridge,
            source_id: constants::browser::SOURCE_ID_MANAGED_CHROMIUM_DEVTOOLS.to_string(),
        },
        kind: ActivityEventKind::UrlObserved,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Url,
            subject_id: browser_subject_id(&normalized.domain),
            display_name: observation
                .title
                .or_else(|| Some(normalized.domain.clone())),
        },
        fields,
        evidence: Vec::new(),
    })
}
