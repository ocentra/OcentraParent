use super::{
    LanPassiveDiscoveryEventKind, LanPassiveDiscoverySource, LanPassiveDiscoveryTriggerReason,
};

mod reasons;
mod sources;

pub fn passive_event_id(
    event_kind: &LanPassiveDiscoveryEventKind,
    source: Option<&LanPassiveDiscoverySource>,
    trigger_reason: &LanPassiveDiscoveryTriggerReason,
    observed_at: &str,
    device_id: Option<&str>,
    scan_session_id: Option<&str>,
) -> String {
    let mut parts = vec![
        String::from("lan-passive"),
        compact_identifier(event_kind_label(event_kind)),
        compact_identifier(trigger_reason_label(trigger_reason)),
        compact_identifier(observed_at),
    ];
    if let Some(source) = source {
        parts.push(compact_identifier(passive_source_label(source)));
    }
    if let Some(device_id) = device_id {
        parts.push(compact_identifier(device_id));
    }
    if let Some(scan_session_id) = scan_session_id {
        parts.push(compact_identifier(scan_session_id));
    }
    parts.join("-")
}

pub fn event_kind_label(event_kind: &LanPassiveDiscoveryEventKind) -> &'static str {
    match event_kind {
        LanPassiveDiscoveryEventKind::PassiveUpdate => "update",
        LanPassiveDiscoveryEventKind::RescanTrigger => "trigger",
    }
}

pub fn trigger_reason_label(trigger_reason: &LanPassiveDiscoveryTriggerReason) -> &'static str {
    reasons::trigger_reason_label(trigger_reason)
}

pub fn passive_source_label(source: &LanPassiveDiscoverySource) -> &'static str {
    sources::passive_source_label(source)
}

pub fn compact_identifier(value: &str) -> String {
    let compacted = value
        .chars()
        .map(|character| match character {
            'a'..='z' | '0'..='9' | '-' | '_' => character,
            'A'..='Z' => character.to_ascii_lowercase(),
            _ => '-',
        })
        .collect::<String>();
    compacted.trim_matches('-').to_string()
}
