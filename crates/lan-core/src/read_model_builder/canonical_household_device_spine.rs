mod builders;
mod merge;
pub mod values;

use std::collections::{HashMap, HashSet};

use builders::{device_from_discovery, device_from_registry};
use merge::{conflicting_source_identity, merge_device};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing::LanTrustedDeviceRegistryEntry;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceConfidence;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceRecord;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceActionKind;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceDecision;
use values::{child_profile_device_id, child_profile_identity_from_canonical};
use values::{evidence_kind_overlaps, option_overlaps, surfaces_for};

const DEDUPE_DECISION_NOTE_PREFIX: &str = "dedupe-decision=";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MergeDecisionState {
    Automatic,
    ManualRequired,
    Forbidden,
    NoMatch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MergeDecisionReason {
    SameCanonicalDeviceId,
    SharedInstallId,
    SharedPairingId,
    SharedStableMac,
    SharedMdnsInstanceName,
    SharedSsdpUdn,
    SharedLocalServiceIdentityAnchor,
    SharedIpAddress,
    SharedHostname,
    SharedVendor,
    SharedDeviceType,
    ConflictingOcentraDeviceId,
    ConflictingChildProfileId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MergeAssessment {
    state: MergeDecisionState,
    score: u16,
    reasons: Vec<MergeDecisionReason>,
}

pub fn canonical_household_devices(
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
    trusted_registry: &[LanTrustedDeviceRegistryEntry],
    household_device_decisions: &[LanHouseholdDeviceDecision],
    observed_at: &str,
) -> Vec<LanCanonicalHouseholdDevice> {
    let assigned_child_profiles = assigned_child_profiles(household_device_decisions);
    let mut devices: Vec<LanCanonicalHouseholdDevice> = Vec::new();
    let mut merge_index: HashMap<String, Vec<usize>> = HashMap::new();

    for discovered in discovered_devices {
        upsert_device(
            &mut devices,
            &mut merge_index,
            device_from_discovery(discovered, observed_at),
            &discovered.child_device,
            &assigned_child_profiles,
        );
    }

    for entry in trusted_registry {
        upsert_device(
            &mut devices,
            &mut merge_index,
            device_from_registry(entry, observed_at),
            &entry.child_device,
            &assigned_child_profiles,
        );
    }

    apply_household_device_decisions(&mut devices, household_device_decisions);
    devices
}

fn upsert_device(
    devices: &mut Vec<LanCanonicalHouseholdDevice>,
    merge_index: &mut HashMap<String, Vec<usize>>,
    device: LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    assigned_child_profiles: &HashMap<String, String>,
) {
    let mut best_automatic: Option<(usize, MergeAssessment)> = None;
    let mut best_blocked: Option<MergeAssessment> = None;

    for index in candidate_indices(merge_index, &device, source_ref) {
        let Some(candidate) = devices.get(index) else {
            continue;
        };
        let assessment =
            assess_merge_candidate(candidate, source_ref, &device, assigned_child_profiles);
        match assessment.state {
            MergeDecisionState::Automatic => {
                if best_automatic
                    .as_ref()
                    .is_none_or(|(_, current)| assessment.score > current.score)
                {
                    best_automatic = Some((index, assessment));
                }
            }
            MergeDecisionState::ManualRequired | MergeDecisionState::Forbidden => {
                if best_blocked
                    .as_ref()
                    .is_none_or(|current| assessment.score > current.score)
                {
                    best_blocked = Some(assessment);
                }
            }
            MergeDecisionState::NoMatch => {}
        }
    }

    if let Some((index, assessment)) = best_automatic {
        let existing = &mut devices[index];
        merge_device(existing, device);
        annotate_merge_assessment(existing, &assessment);
        index_device(merge_index, existing, source_ref, index);
        return;
    }

    let mut device = device;
    if let Some(assessment) = best_blocked {
        annotate_merge_assessment(&mut device, &assessment);
    }

    let index = devices.len();
    index_device(merge_index, &device, source_ref, index);
    devices.push(device);
}

fn candidate_indices(
    merge_index: &HashMap<String, Vec<usize>>,
    device: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
) -> Vec<usize> {
    let mut seen = HashSet::new();
    let mut indices = Vec::new();

    for key in merge_candidate_keys(device, source_ref) {
        let Some(indexed) = merge_index.get(&key) else {
            continue;
        };
        for index in indexed {
            if seen.insert(*index) {
                indices.push(*index);
            }
        }
    }

    indices
}

fn index_device(
    merge_index: &mut HashMap<String, Vec<usize>>,
    device: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    index: usize,
) {
    for key in merge_candidate_keys(device, source_ref) {
        let indices = merge_index.entry(key).or_default();
        if !indices.contains(&index) {
            indices.push(index);
        }
    }
}

fn merge_candidate_keys(
    device: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
) -> HashSet<String> {
    let mut keys = HashSet::new();

    push_normalized_key(&mut keys, "canonical", &device.canonical_device_id);
    if let Some(mac_address) = device.network_identity.mac_address.as_deref() {
        push_normalized_key(&mut keys, "mac", mac_address);
    }
    if let Some(mac_address) = source_ref.mac_address.as_deref() {
        push_normalized_key(&mut keys, "mac", mac_address);
    }
    if let Some(ip_address) = source_ref.ip_address.as_deref() {
        push_normalized_key(&mut keys, "ip", ip_address);
    }
    for ip_address in &device.network_identity.ip_addresses {
        push_normalized_key(&mut keys, "ip", ip_address);
    }
    if let Some(hostname) = device.network_identity.hostname.as_deref() {
        push_normalized_key(&mut keys, "hostname", hostname);
    }
    for record in &device.network_identity.evidence_records {
        match record.evidence_kind {
            LanDiscoveryEvidenceKind::InstallId => {
                push_normalized_key(&mut keys, "install", &record.normalized_value);
            }
            LanDiscoveryEvidenceKind::PairingId => {
                push_normalized_key(&mut keys, "pairing", &record.normalized_value);
            }
            LanDiscoveryEvidenceKind::TrustedRegistry => {
                push_normalized_key(&mut keys, "trusted", &record.normalized_value);
            }
            LanDiscoveryEvidenceKind::Vendor => {
                push_normalized_key(&mut keys, "vendor", &record.normalized_value);
            }
            LanDiscoveryEvidenceKind::ServiceProbeHint => {
                push_service_hint_key(&mut keys, record);
            }
            LanDiscoveryEvidenceKind::MacAddress => {
                push_normalized_key(&mut keys, "mac", &record.normalized_value);
            }
            LanDiscoveryEvidenceKind::IpAddress => {
                push_normalized_key(&mut keys, "ip", &record.normalized_value);
            }
            LanDiscoveryEvidenceKind::Hostname => {
                push_normalized_key(&mut keys, "hostname", &record.normalized_value);
            }
            LanDiscoveryEvidenceKind::ChildAgentPresence
            | LanDiscoveryEvidenceKind::HistoricalIdentityHint
            | LanDiscoveryEvidenceKind::Interface
            | LanDiscoveryEvidenceKind::ParentDecision
            | LanDiscoveryEvidenceKind::Route
            | LanDiscoveryEvidenceKind::RouterClassification => {}
        }
    }

    keys
}

fn push_service_hint_key(keys: &mut HashSet<String>, record: &LanDiscoveryEvidenceRecord) {
    for prefix in [
        "mdns-instance-name:",
        "ssdp-udn:",
        "mdns-service-type:",
        "ssdp-device-type:",
    ] {
        if record
            .value
            .get(..prefix.len())
            .is_some_and(|value| value.eq_ignore_ascii_case(prefix))
        {
            push_normalized_key(keys, prefix, &record.normalized_value);
        }
    }
}

fn push_normalized_key(keys: &mut HashSet<String>, namespace: &str, value: &str) {
    let normalized = value.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return;
    }
    keys.insert(format!("{namespace}:{normalized}"));
}

fn assess_merge_candidate(
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    device: &LanCanonicalHouseholdDevice,
    assigned_child_profiles: &HashMap<String, String>,
) -> MergeAssessment {
    let authoritative_identity_overlap = authoritative_mac_overlap(existing, source_ref, device)
        || authoritative_ip_overlap(existing, source_ref, device);
    let local_service_identity_overlap =
        authoritative_identity_overlap && has_local_service_identity_anchor(existing, device);
    let mut reasons = merge_reasons(
        existing,
        source_ref,
        device,
        authoritative_identity_overlap,
        local_service_identity_overlap,
    );
    let score = merge_score(&reasons);
    let strong_hint_overlap =
        strong_service_hint_overlap(existing, device, &["mdns-instance-name:", "ssdp-udn:"]);
    if conflicting_child_profile_identity(existing, source_ref, device, assigned_child_profiles) {
        push_merge_reason(&mut reasons, MergeDecisionReason::ConflictingChildProfileId);
        return MergeAssessment {
            state: MergeDecisionState::Forbidden,
            score,
            reasons,
        };
    }
    let has_non_manual_overlap = reasons
        .iter()
        .any(|reason| !merge_reason_is_manual_required(*reason));
    if conflicting_source_identity(existing, device)
        && (authoritative_identity_overlap || has_non_manual_overlap)
        && !strong_hint_overlap
        && !local_service_identity_overlap
    {
        push_merge_reason(
            &mut reasons,
            MergeDecisionReason::ConflictingOcentraDeviceId,
        );
        return MergeAssessment {
            state: MergeDecisionState::Forbidden,
            score,
            reasons,
        };
    }
    let automatic_match = existing.canonical_device_id == device.canonical_device_id
        || option_overlaps(
            existing.network_identity.mac_address.as_ref(),
            device.network_identity.mac_address.as_ref(),
        )
        || option_overlaps(
            existing.network_identity.mac_address.as_ref(),
            source_ref.mac_address.as_ref(),
        )
        || evidence_kind_overlaps(
            existing,
            device,
            &[
                LanDiscoveryEvidenceKind::MacAddress,
                LanDiscoveryEvidenceKind::InstallId,
                LanDiscoveryEvidenceKind::PairingId,
                LanDiscoveryEvidenceKind::ChildAgentPresence,
                LanDiscoveryEvidenceKind::TrustedRegistry,
            ],
        )
        || strong_hint_overlap
        || local_service_identity_overlap
        || authoritative_identity_overlap;

    if automatic_match {
        return MergeAssessment {
            state: MergeDecisionState::Automatic,
            score,
            reasons,
        };
    }

    if reasons
        .iter()
        .any(|reason| merge_reason_is_manual_required(*reason))
    {
        return MergeAssessment {
            state: MergeDecisionState::ManualRequired,
            score,
            reasons,
        };
    }

    MergeAssessment {
        state: MergeDecisionState::NoMatch,
        score: 0,
        reasons,
    }
}

fn merge_reasons(
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    device: &LanCanonicalHouseholdDevice,
    authoritative_identity_overlap: bool,
    local_service_identity_overlap: bool,
) -> Vec<MergeDecisionReason> {
    let mut reasons = Vec::new();
    if existing.canonical_device_id == device.canonical_device_id {
        push_merge_reason(&mut reasons, MergeDecisionReason::SameCanonicalDeviceId);
    }
    if option_overlaps(
        existing.network_identity.mac_address.as_ref(),
        device.network_identity.mac_address.as_ref(),
    ) || option_overlaps(
        existing.network_identity.mac_address.as_ref(),
        source_ref.mac_address.as_ref(),
    ) {
        push_merge_reason(&mut reasons, MergeDecisionReason::SharedStableMac);
    }
    if evidence_kind_overlaps(existing, device, &[LanDiscoveryEvidenceKind::InstallId]) {
        push_merge_reason(&mut reasons, MergeDecisionReason::SharedInstallId);
    }
    if evidence_kind_overlaps(existing, device, &[LanDiscoveryEvidenceKind::PairingId]) {
        push_merge_reason(&mut reasons, MergeDecisionReason::SharedPairingId);
    }
    if strong_service_hint_overlap(existing, device, &["mdns-instance-name:"]) {
        push_merge_reason(&mut reasons, MergeDecisionReason::SharedMdnsInstanceName);
    }
    if strong_service_hint_overlap(existing, device, &["ssdp-udn:"]) {
        push_merge_reason(&mut reasons, MergeDecisionReason::SharedSsdpUdn);
    }
    if local_service_identity_overlap && authoritative_identity_overlap {
        push_merge_reason(
            &mut reasons,
            MergeDecisionReason::SharedLocalServiceIdentityAnchor,
        );
    }
    if shared_ip_overlap(existing, source_ref, device) {
        push_merge_reason(&mut reasons, MergeDecisionReason::SharedIpAddress);
    }
    if shared_hostname_overlap(existing, device) {
        push_merge_reason(&mut reasons, MergeDecisionReason::SharedHostname);
    }
    if shared_vendor_overlap(existing, device) {
        push_merge_reason(&mut reasons, MergeDecisionReason::SharedVendor);
    }
    if shared_device_type_overlap(existing, device) {
        push_merge_reason(&mut reasons, MergeDecisionReason::SharedDeviceType);
    }
    reasons
}

fn push_merge_reason(reasons: &mut Vec<MergeDecisionReason>, reason: MergeDecisionReason) {
    if !reasons.contains(&reason) {
        reasons.push(reason);
    }
}

fn merge_score(reasons: &[MergeDecisionReason]) -> u16 {
    reasons
        .iter()
        .map(|reason| merge_reason_score(*reason))
        .sum()
}

fn merge_reason_score(reason: MergeDecisionReason) -> u16 {
    match reason {
        MergeDecisionReason::SameCanonicalDeviceId => 120,
        MergeDecisionReason::SharedInstallId => 110,
        MergeDecisionReason::SharedPairingId => 110,
        MergeDecisionReason::SharedStableMac => 100,
        MergeDecisionReason::SharedMdnsInstanceName => 95,
        MergeDecisionReason::SharedSsdpUdn => 95,
        MergeDecisionReason::SharedLocalServiceIdentityAnchor => 90,
        MergeDecisionReason::SharedIpAddress => 25,
        MergeDecisionReason::SharedHostname => 20,
        MergeDecisionReason::SharedVendor => 10,
        MergeDecisionReason::SharedDeviceType => 12,
        MergeDecisionReason::ConflictingOcentraDeviceId => 0,
        MergeDecisionReason::ConflictingChildProfileId => 0,
    }
}

fn merge_reason_is_manual_required(reason: MergeDecisionReason) -> bool {
    matches!(
        reason,
        MergeDecisionReason::SharedIpAddress
            | MergeDecisionReason::SharedHostname
            | MergeDecisionReason::SharedVendor
            | MergeDecisionReason::SharedDeviceType
    )
}

fn shared_ip_overlap(
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    incoming: &LanCanonicalHouseholdDevice,
) -> bool {
    let incoming_ip_matches = incoming
        .network_identity
        .ip_addresses
        .iter()
        .any(|ip| existing.network_identity.ip_addresses.contains(ip));
    let source_ip_matches = source_ref
        .ip_address
        .as_ref()
        .is_some_and(|ip| existing.network_identity.ip_addresses.contains(ip));
    incoming_ip_matches || source_ip_matches
}

fn shared_hostname_overlap(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
) -> bool {
    existing
        .network_identity
        .hostname
        .as_ref()
        .zip(incoming.network_identity.hostname.as_ref())
        .is_some_and(|(left, right)| left.eq_ignore_ascii_case(right))
}

fn shared_vendor_overlap(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
) -> bool {
    evidence_overlap_by_kind(existing, incoming, &LanDiscoveryEvidenceKind::Vendor)
}

fn shared_device_type_overlap(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
) -> bool {
    strong_service_hint_overlap(
        existing,
        incoming,
        &["mdns-service-type:", "ssdp-device-type:"],
    )
}

fn evidence_overlap_by_kind(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
    evidence_kind: &LanDiscoveryEvidenceKind,
) -> bool {
    existing
        .network_identity
        .evidence_records
        .iter()
        .filter(|record| record.evidence_kind == *evidence_kind)
        .any(|existing_record| {
            incoming
                .network_identity
                .evidence_records
                .iter()
                .filter(|record| record.evidence_kind == *evidence_kind)
                .any(|incoming_record| {
                    existing_record
                        .normalized_value
                        .eq_ignore_ascii_case(&incoming_record.normalized_value)
                })
        })
}

fn annotate_merge_assessment(
    device: &mut LanCanonicalHouseholdDevice,
    assessment: &MergeAssessment,
) {
    if assessment.state == MergeDecisionState::NoMatch || assessment.reasons.is_empty() {
        return;
    }
    if assessment.state != MergeDecisionState::Automatic {
        device.network_identity.confidence = LanCanonicalHouseholdDeviceConfidence::ManualRequired;
    }
    let note = merge_assessment_note(assessment);
    let mut annotated = false;
    for reason in &assessment.reasons {
        let mut reason_annotated = false;
        for record in device
            .network_identity
            .evidence_records
            .iter_mut()
            .filter(|record| merge_reason_matches_record(*reason, record))
        {
            append_merge_note(&mut record.note, &note);
            reason_annotated = true;
            annotated = true;
        }
        if !reason_annotated {
            continue;
        }
    }
    if !annotated {
        if let Some(record) = device.network_identity.evidence_records.first_mut() {
            append_merge_note(&mut record.note, &note);
        }
    }
}

fn merge_assessment_note(assessment: &MergeAssessment) -> String {
    let reasons = assessment
        .reasons
        .iter()
        .map(|reason| merge_reason_label(*reason))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{DEDUPE_DECISION_NOTE_PREFIX}{} score={} reasons={reasons}",
        merge_state_label(assessment.state),
        assessment.score
    )
}

fn merge_state_label(state: MergeDecisionState) -> &'static str {
    match state {
        MergeDecisionState::Automatic => "automatic",
        MergeDecisionState::ManualRequired => "manual-required",
        MergeDecisionState::Forbidden => "forbidden",
        MergeDecisionState::NoMatch => "no-match",
    }
}

fn merge_reason_label(reason: MergeDecisionReason) -> &'static str {
    match reason {
        MergeDecisionReason::SameCanonicalDeviceId => "same-canonical-device-id",
        MergeDecisionReason::SharedInstallId => "shared-install-id",
        MergeDecisionReason::SharedPairingId => "shared-pairing-id",
        MergeDecisionReason::SharedStableMac => "shared-stable-mac",
        MergeDecisionReason::SharedMdnsInstanceName => "shared-mdns-instance-name",
        MergeDecisionReason::SharedSsdpUdn => "shared-ssdp-udn",
        MergeDecisionReason::SharedLocalServiceIdentityAnchor => {
            "shared-local-service-identity-anchor"
        }
        MergeDecisionReason::SharedIpAddress => "shared-ip-address",
        MergeDecisionReason::SharedHostname => "shared-hostname",
        MergeDecisionReason::SharedVendor => "shared-vendor",
        MergeDecisionReason::SharedDeviceType => "shared-device-type",
        MergeDecisionReason::ConflictingOcentraDeviceId => "conflicting-ocentra-device-id",
        MergeDecisionReason::ConflictingChildProfileId => "conflicting-child-profile-id",
    }
}

fn merge_reason_matches_record(
    reason: MergeDecisionReason,
    record: &LanDiscoveryEvidenceRecord,
) -> bool {
    match reason {
        MergeDecisionReason::SameCanonicalDeviceId => {
            matches!(
                record.evidence_kind,
                LanDiscoveryEvidenceKind::ChildAgentPresence
                    | LanDiscoveryEvidenceKind::TrustedRegistry
            )
        }
        MergeDecisionReason::SharedInstallId => {
            record.evidence_kind == LanDiscoveryEvidenceKind::InstallId
        }
        MergeDecisionReason::SharedPairingId => {
            record.evidence_kind == LanDiscoveryEvidenceKind::PairingId
        }
        MergeDecisionReason::SharedStableMac => {
            record.evidence_kind == LanDiscoveryEvidenceKind::MacAddress
        }
        MergeDecisionReason::SharedMdnsInstanceName => {
            service_probe_prefix_match(record, "mdns-instance-name:")
        }
        MergeDecisionReason::SharedSsdpUdn => service_probe_prefix_match(record, "ssdp-udn:"),
        MergeDecisionReason::SharedLocalServiceIdentityAnchor
        | MergeDecisionReason::SharedIpAddress => {
            record.evidence_kind == LanDiscoveryEvidenceKind::IpAddress
        }
        MergeDecisionReason::SharedHostname => {
            record.evidence_kind == LanDiscoveryEvidenceKind::Hostname
        }
        MergeDecisionReason::SharedVendor => {
            record.evidence_kind == LanDiscoveryEvidenceKind::Vendor
        }
        MergeDecisionReason::SharedDeviceType => {
            service_probe_prefix_match(record, "mdns-service-type:")
                || service_probe_prefix_match(record, "ssdp-device-type:")
        }
        MergeDecisionReason::ConflictingOcentraDeviceId
        | MergeDecisionReason::ConflictingChildProfileId => false,
    }
}

fn service_probe_prefix_match(record: &LanDiscoveryEvidenceRecord, prefix: &str) -> bool {
    record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint
        && record
            .value
            .get(..prefix.len())
            .is_some_and(|value| value.eq_ignore_ascii_case(prefix))
}

fn append_merge_note(note: &mut Option<String>, merge_note: &str) {
    match note {
        Some(existing) => {
            if !existing.contains(merge_note) {
                existing.push_str(" | ");
                existing.push_str(merge_note);
            }
        }
        None => *note = Some(merge_note.to_string()),
    }
}

fn conflicting_child_profile_identity(
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    incoming: &LanCanonicalHouseholdDevice,
    assigned_child_profiles: &HashMap<String, String>,
) -> bool {
    let Some(existing_child_profile_id) =
        assigned_child_profile_identity(existing, assigned_child_profiles).or_else(|| {
            child_profile_identity_from_canonical(&existing.canonical_device_id)
                .map(ToOwned::to_owned)
        })
    else {
        return false;
    };
    let Some(incoming_child_profile_id) =
        assigned_child_profile_identity(incoming, assigned_child_profiles)
            .or_else(|| child_profile_device_id(source_ref))
    else {
        return false;
    };
    existing_child_profile_id != incoming_child_profile_id
}

fn assigned_child_profile_identity(
    device: &LanCanonicalHouseholdDevice,
    assigned_child_profiles: &HashMap<String, String>,
) -> Option<String> {
    assigned_child_profiles
        .get(&device.canonical_device_id)
        .cloned()
}

fn authoritative_mac_overlap(
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    device: &LanCanonicalHouseholdDevice,
) -> bool {
    let mac_matches = option_overlaps(
        existing.network_identity.mac_address.as_ref(),
        device.network_identity.mac_address.as_ref(),
    ) || option_overlaps(
        existing.network_identity.mac_address.as_ref(),
        source_ref.mac_address.as_ref(),
    );
    mac_matches
        && (has_agent_or_registry_evidence(existing) || has_agent_or_registry_evidence(device))
}

fn authoritative_ip_overlap(
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    device: &LanCanonicalHouseholdDevice,
) -> bool {
    let ip_matches = source_ref
        .ip_address
        .as_ref()
        .map(|ip| existing.network_identity.ip_addresses.contains(ip))
        .unwrap_or(false);
    ip_matches
        && (has_agent_or_registry_evidence(existing) || has_agent_or_registry_evidence(device))
}

fn has_agent_or_registry_evidence(device: &LanCanonicalHouseholdDevice) -> bool {
    device.classification == LanCanonicalHouseholdDeviceClassification::ChildAgent
        || device.child_agent_inventory.is_some()
        || device.source_labels.iter().any(|source| {
            matches!(
                source,
                LanCanonicalHouseholdDeviceSource::LocalService
                    | LanCanonicalHouseholdDeviceSource::TrustedRegistry
            )
        })
}

fn has_local_service_identity_anchor(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
) -> bool {
    existing
        .source_labels
        .iter()
        .chain(incoming.source_labels.iter())
        .any(|source| matches!(source, LanCanonicalHouseholdDeviceSource::LocalService))
}

fn strong_service_hint_overlap(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
    prefixes: &[&str],
) -> bool {
    existing
        .network_identity
        .evidence_records
        .iter()
        .filter(|record| strong_service_hint_record(record, prefixes))
        .any(|existing_record| {
            incoming
                .network_identity
                .evidence_records
                .iter()
                .filter(|record| strong_service_hint_record(record, prefixes))
                .any(|incoming_record| {
                    existing_record
                        .normalized_value
                        .eq_ignore_ascii_case(&incoming_record.normalized_value)
                })
        })
}

fn strong_service_hint_record(record: &LanDiscoveryEvidenceRecord, prefixes: &[&str]) -> bool {
    record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint
        && record.confidence == LanDiscoveryEvidenceConfidence::Strong
        && prefixes.iter().any(|prefix| {
            record
                .value
                .get(..prefix.len())
                .is_some_and(|value| value.eq_ignore_ascii_case(prefix))
        })
}

fn assigned_child_profiles(decisions: &[LanHouseholdDeviceDecision]) -> HashMap<String, String> {
    let mut assignments = HashMap::new();

    for decision in decisions {
        if decision.revoked_at.is_some() {
            continue;
        }

        match decision.action_kind {
            LanHouseholdDeviceActionKind::Assign | LanHouseholdDeviceActionKind::Trust => {
                let Some(child_profile_id) =
                    normalized_child_profile_id(decision.child_profile_id.as_deref())
                else {
                    continue;
                };
                assignments.insert(decision.canonical_device_id.clone(), child_profile_id);
            }
            LanHouseholdDeviceActionKind::Ignore | LanHouseholdDeviceActionKind::Revoke => {
                assignments.remove(&decision.canonical_device_id);
            }
            LanHouseholdDeviceActionKind::Rename | LanHouseholdDeviceActionKind::Restore => {}
        }
    }

    assignments
}

fn normalized_child_profile_id(child_profile_id: Option<&str>) -> Option<String> {
    let normalized = child_profile_id?
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect::<String>();
    (!normalized.is_empty()).then_some(normalized)
}

fn apply_household_device_decisions(
    devices: &mut [LanCanonicalHouseholdDevice],
    decisions: &[LanHouseholdDeviceDecision],
) {
    for decision in decisions
        .iter()
        .filter(|decision| decision.revoked_at.is_none())
    {
        if let Some(device) = devices
            .iter_mut()
            .find(|device| device.canonical_device_id == decision.canonical_device_id)
        {
            apply_household_device_decision(device, decision);
        }
    }
}

fn apply_household_device_decision(
    device: &mut LanCanonicalHouseholdDevice,
    decision: &LanHouseholdDeviceDecision,
) {
    if let Some(display_name) = decision
        .display_name
        .as_ref()
        .filter(|value| !value.is_empty())
    {
        device.display_name = display_name.clone();
        if let Some(inventory) = device.child_agent_inventory.as_mut() {
            inventory.device_name = display_name.clone();
        }
    }

    match &decision.action_kind {
        LanHouseholdDeviceActionKind::Ignore => {
            device.discovery_state = LanPairingProductionDiscoveryState::Revoked;
            device.trust_state = LanPairingTrustState::Revoked;
            device.enrollable = false;
            device.route_id = None;
            device.route_state = ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState::Unavailable;
            device.policy_target_surfaces = surfaces_for(false);
        }
        LanHouseholdDeviceActionKind::Revoke => {
            device.discovery_state = LanPairingProductionDiscoveryState::Revoked;
            device.trust_state = LanPairingTrustState::Revoked;
            device.enrollable = false;
            device.route_id = None;
            device.route_state = ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState::Unavailable;
            device.policy_target_surfaces = surfaces_for(false);
        }
        LanHouseholdDeviceActionKind::Restore => {
            if device.discovery_state == LanPairingProductionDiscoveryState::Revoked {
                device.discovery_state = LanPairingProductionDiscoveryState::Discovered;
            }
            if device.trust_state == LanPairingTrustState::Revoked {
                device.trust_state = LanPairingTrustState::Unpaired;
            }
            device.enrollable =
                device.classification == LanCanonicalHouseholdDeviceClassification::ChildAgent;
            device.policy_target_surfaces = surfaces_for(device.enrollable);
        }
        LanHouseholdDeviceActionKind::Assign | LanHouseholdDeviceActionKind::Trust => {
            device.trust_state = LanPairingTrustState::Paired;
            if let Some(inventory) = device.child_agent_inventory.as_mut() {
                inventory.pairing_trust_state = LanPairingTrustState::Paired;
            }
        }
        LanHouseholdDeviceActionKind::Rename => {}
    }

    push_parent_decision_evidence(device, decision);
}

fn push_parent_decision_evidence(
    device: &mut LanCanonicalHouseholdDevice,
    decision: &LanHouseholdDeviceDecision,
) {
    let merge_key = parent_decision_merge_key(&decision.action_id);
    if device
        .network_identity
        .evidence_records
        .iter()
        .any(|record| record.merge_key == merge_key)
    {
        return;
    }
    device
        .network_identity
        .evidence_records
        .push(LanDiscoveryEvidenceRecord {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            evidence_id: parent_decision_evidence_id(&decision.action_id),
            source: LanDiscoveryEvidenceSource::ParentAssignment,
            evidence_kind: LanDiscoveryEvidenceKind::ParentDecision,
            device_id: decision.canonical_device_id.clone(),
            value: household_action_value(&decision.action_kind).to_string(),
            normalized_value: household_action_value(&decision.action_kind).to_string(),
            first_seen_at: decision.decided_at.clone(),
            last_seen_at: decision.decided_at.clone(),
            expires_at: None,
            confidence: parent_decision_confidence(&decision.action_kind),
            merge_key,
            note: decision.display_name.clone(),
        });
}

fn parent_decision_merge_key(action_id: &str) -> String {
    let mut key = String::from(constants::lan_pairing::LAN_EVIDENCE_KEY_PARENT_DECISION_PREFIX);
    key.push_str(action_id);
    key
}

fn parent_decision_evidence_id(action_id: &str) -> String {
    let mut id = String::from(constants::lan_pairing::LAN_EVIDENCE_ID_PREFIX);
    id.push_str(action_id);
    id
}

fn household_action_value(action_kind: &LanHouseholdDeviceActionKind) -> &'static str {
    match action_kind {
        LanHouseholdDeviceActionKind::Assign => constants::lan_pairing::HOUSEHOLD_ACTION_ASSIGN,
        LanHouseholdDeviceActionKind::Rename => constants::lan_pairing::HOUSEHOLD_ACTION_RENAME,
        LanHouseholdDeviceActionKind::Ignore => constants::lan_pairing::HOUSEHOLD_ACTION_IGNORE,
        LanHouseholdDeviceActionKind::Revoke => constants::lan_pairing::HOUSEHOLD_ACTION_REVOKE,
        LanHouseholdDeviceActionKind::Restore => constants::lan_pairing::HOUSEHOLD_ACTION_RESTORE,
        LanHouseholdDeviceActionKind::Trust => constants::lan_pairing::HOUSEHOLD_ACTION_TRUST,
    }
}

fn parent_decision_confidence(
    action_kind: &LanHouseholdDeviceActionKind,
) -> LanDiscoveryEvidenceConfidence {
    if matches!(
        action_kind,
        LanHouseholdDeviceActionKind::Ignore | LanHouseholdDeviceActionKind::Revoke
    ) {
        LanDiscoveryEvidenceConfidence::Rejected
    } else {
        LanDiscoveryEvidenceConfidence::Confirmed
    }
}
