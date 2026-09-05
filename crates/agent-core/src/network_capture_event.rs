use ocentra_parent_agent_protocol::activity::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityEvidenceKind, ActivityEvidenceRef, ActivityObserver,
    ActivitySource, ActivitySubject, ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::constants;

use crate::network_capture::{collect_network_snapshot, NetworkObservation};
use crate::network_capture_event_fields::{
    network_display_name, network_fields, network_subject_id,
};

#[derive(Clone, Debug, PartialEq)]
pub struct NetworkCaptureResult {
    observation: NetworkObservation,
    activity_event: ActivityEvent,
}

impl NetworkCaptureResult {
    pub fn observation(&self) -> &NetworkObservation {
        &self.observation
    }

    pub fn activity_event(&self) -> &ActivityEvent {
        &self.activity_event
    }

    pub fn into_parts(self) -> (NetworkObservation, ActivityEvent) {
        (self.observation, self.activity_event)
    }
}

pub fn network_snapshot_capture_results(
    observed_at: &str,
    limit: usize,
) -> Vec<NetworkCaptureResult> {
    collect_network_snapshot(limit)
        .into_iter()
        .enumerate()
        .map(|(index, observation)| NetworkCaptureResult {
            activity_event: network_observation_event(observation.clone(), observed_at, index),
            observation,
        })
        .collect()
}

pub fn network_snapshot_events(observed_at: &str, limit: usize) -> Vec<ActivityEvent> {
    network_snapshot_capture_results(observed_at, limit)
        .into_iter()
        .map(|capture| capture.activity_event)
        .collect()
}

pub fn network_observation_event(
    observation: NetworkObservation,
    observed_at: &str,
    sequence_index: usize,
) -> ActivityEvent {
    let NetworkObservation {
        status,
        protocol,
        local_ip,
        local_port,
        destination_ip,
        destination_port,
        destination_domain,
        tcp_state,
        pid,
        process_name,
        associated_pid_count,
    } = observation;
    let observation = NetworkObservation {
        status,
        protocol,
        local_ip,
        local_port,
        destination_ip,
        destination_port,
        destination_domain,
        tcp_state,
        pid,
        process_name,
        associated_pid_count,
    };
    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: network_event_id(&observation, observed_at, sequence_index),
        observed_at: observed_at.to_string(),
        source: ActivitySource {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: std::env::consts::OS.to_string(),
            observer: ActivityObserver::WindowsNetwork,
            source_id: constants::activity_capture::WINDOWS_NETWORK_SOURCE_ID.to_string(),
        },
        kind: ActivityEventKind::DomainObserved,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Domain,
            subject_id: network_subject_id(&observation),
            display_name: network_display_name(&observation),
        },
        fields: network_fields(&observation),
        evidence: network_evidence_refs(&observation, observed_at, sequence_index),
    }
}

fn network_evidence_refs(
    observation: &NetworkObservation,
    observed_at: &str,
    sequence_index: usize,
) -> Vec<ActivityEvidenceRef> {
    vec![ActivityEvidenceRef {
        evidence_id: network_evidence_id(observation, observed_at, sequence_index),
        kind: ActivityEvidenceKind::LocalDbRow,
        digest: None,
        uri: Some(network_evidence_uri(
            observation,
            observed_at,
            sequence_index,
        )),
    }]
}

fn network_evidence_id(
    observation: &NetworkObservation,
    observed_at: &str,
    sequence_index: usize,
) -> String {
    let mut evidence_id = String::from(constants::activity_capture::NETWORK_EVIDENCE_ID_PREFIX);
    evidence_id.push_str(observation.status.as_protocol_str());
    evidence_id.push(constants::delimiter::HYPHEN);
    evidence_id.push_str(&sequence_index.to_string());
    evidence_id.push(constants::delimiter::HYPHEN);
    evidence_id.push_str(observed_at);
    evidence_id
}

fn network_evidence_uri(
    observation: &NetworkObservation,
    observed_at: &str,
    sequence_index: usize,
) -> String {
    let mut uri = String::from(constants::activity_capture::NETWORK_EVIDENCE_URI_PREFIX);
    uri.push_str(observation.status.as_protocol_str());
    uri.push(constants::delimiter::SLASH);
    uri.push_str(&sequence_index.to_string());
    uri.push(constants::delimiter::SLASH);
    uri.push_str(observed_at);
    uri
}

fn network_event_id(
    observation: &NetworkObservation,
    observed_at: &str,
    sequence_index: usize,
) -> String {
    let mut event_id = String::from(constants::activity_capture::NETWORK_EVENT_ID_PREFIX);
    event_id.push_str(observation.status.as_protocol_str());
    event_id.push(constants::delimiter::HYPHEN);
    event_id.push_str(&sequence_index.to_string());
    event_id.push(constants::delimiter::HYPHEN);
    event_id.push_str(observed_at);
    event_id
}
