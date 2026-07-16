use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdNetworkIdentity;

use super::value_support::{
    confidence_for_mac_identity, known_hostname, offline_at_for, stale_at_for,
};
use super::{evidence::evidence_records_for, evidence::EvidenceRecordsInput, NetworkIdentityInput};
use crate::mac_identity::assess_mac_address;

pub(super) fn network_identity_for(
    input: NetworkIdentityInput<'_>,
) -> LanCanonicalHouseholdNetworkIdentity {
    let NetworkIdentityInput {
        device,
        pairing_id,
        reachability,
        confidence,
        source,
        evidence_sources,
        hint_sources,
        service_identity_probe_evidence,
        observed_at,
    } = input;
    let mac_assessment = assess_mac_address(device.mac_address.as_deref());
    let stable_mac_assessment = mac_assessment
        .as_ref()
        .filter(|assessment| assessment.stable_identity_key_allowed());
    let effective_confidence = confidence_for_mac_identity(confidence, mac_assessment.as_ref());
    LanCanonicalHouseholdNetworkIdentity {
        hostname: known_hostname(device),
        ip_addresses: device.ip_address.clone().into_iter().collect(),
        mac_address: stable_mac_assessment
            .as_ref()
            .and_then(|assessment| assessment.normalized_owned()),
        mac_vendor: stable_mac_assessment
            .as_ref()
            .and_then(|assessment| assessment.vendor_name())
            .map(str::to_string),
        network_interfaces: device.network_interface.clone().into_iter().collect(),
        stale_at: stale_at_for(&reachability, observed_at),
        offline_at: offline_at_for(&reachability, observed_at),
        reachability,
        confidence: effective_confidence,
        evidence_records: evidence_records_for(&EvidenceRecordsInput {
            device,
            pairing_id,
            source,
            evidence_sources,
            hint_sources,
            service_identity_probe_evidence,
            observed_at,
            mac_assessment: mac_assessment.as_ref(),
        }),
    }
}
