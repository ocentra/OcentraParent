use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind, BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel,
    BrowserCustodyLabel, BrowserFamily, ACTIVITY_SCHEMA_VERSION,
};

use crate::browser_bridge_fields::{
    base_browser_fields, insert_optional_text, normalized_browser_url,
};
use crate::browser_bridge_ids::{browser_event_id, browser_evidence_id, browser_subject_id};

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
    pub url: String,
    pub title: Option<String>,
    pub capability_status: BrowserCapabilityStatus,
    pub custody_label: BrowserCustodyLabel,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrowserBridgeEventError {
    InvalidUrl,
}

pub fn browser_tab_observation_event(
    observation: BrowserBridgeTargetObservation,
    observed_at: &str,
    fresh_until: &str,
    sequence_index: usize,
) -> Result<ActivityEvent, BrowserBridgeEventError> {
    let normalized =
        normalized_browser_url(&observation.url).ok_or(BrowserBridgeEventError::InvalidUrl)?;
    let evidence_id = browser_evidence_id(&observation, observed_at, sequence_index);
    let mut fields = base_browser_fields(&observation, &evidence_id, fresh_until, &normalized);
    insert_optional_text(&mut fields, constants::field::TAB_ID, &observation.tab_id);
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
