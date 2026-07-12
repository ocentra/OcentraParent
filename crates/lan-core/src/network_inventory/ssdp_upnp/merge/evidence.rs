use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanServiceIdentityProbeEvidence, LanServiceIdentityProbeEvidenceKind,
};

use super::super::http::text::parse_udn;
use super::super::SsdpDiscoveryRecord;

pub(super) fn ssdp_hint_evidence(
    record: &SsdpDiscoveryRecord,
    selected_interface: Option<&str>,
) -> Vec<LanServiceIdentityProbeEvidence> {
    let mut evidence = Vec::new();
    let parsed_udn = parse_udn(&record.response.usn);
    if let Some(udn) = record.response.udn.as_deref().or(parsed_udn.as_deref()) {
        push_ssdp_hint(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::SsdpUdn,
            udn,
            selected_interface.map(str::to_string),
        );
    }
    if let Some(device_type) = record
        .description
        .as_ref()
        .and_then(|description| description.device_type.as_deref())
        .or(record.response.device_type.as_deref())
    {
        push_ssdp_hint(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::SsdpDeviceType,
            device_type,
            selected_interface.map(str::to_string),
        );
    }
    evidence
}

pub(super) fn push_ssdp_hint(
    records: &mut Vec<LanServiceIdentityProbeEvidence>,
    evidence_kind: LanServiceIdentityProbeEvidenceKind,
    value: &str,
    selected_interface: Option<String>,
) {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return;
    }
    if let Some(existing) = records.iter_mut().find(|record| {
        record.evidence_kind == evidence_kind && record.value.eq_ignore_ascii_case(trimmed)
    }) {
        if existing.selected_interface.is_none() {
            existing.selected_interface = selected_interface;
        }
        return;
    }
    records.push(LanServiceIdentityProbeEvidence {
        evidence_kind,
        value: trimmed.to_string(),
        selected_interface,
    });
}
