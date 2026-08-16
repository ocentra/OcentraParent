#[path = "evidence_agent.rs"]
mod evidence_agent;
#[path = "evidence_auxiliary.rs"]
mod evidence_auxiliary;
#[path = "evidence_context.rs"]
mod evidence_context;
#[path = "evidence_identity.rs"]
mod evidence_identity;
#[path = "evidence_network.rs"]
mod evidence_network;
#[path = "evidence_record.rs"]
mod evidence_record;
#[path = "evidence_service_probe.rs"]
mod evidence_service_probe;
#[path = "evidence_vendor.rs"]
mod evidence_vendor;
#[path = "evidence_weak_name.rs"]
mod evidence_weak_name;

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceRecord;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidence;

use crate::mac_identity::LanMacIdentityAssessment;

use evidence_agent::push_agent_evidence;
use evidence_auxiliary::{
    push_fallback_evidence, push_hint_evidence, push_router_evidence,
    push_trusted_registry_evidence,
};
use evidence_context::evidence_context_for;
use evidence_identity::push_strong_identity_evidence;
use evidence_network::push_network_identity_evidence;
use evidence_service_probe::push_service_probe_evidence;
use evidence_vendor::push_vendor_evidence;
use evidence_weak_name::push_weak_name_evidence;

struct EvidenceContext {
    source: LanDiscoveryEvidenceSource,
    confidence: LanDiscoveryEvidenceConfidence,
}

pub(super) struct EvidenceRecordsInput<'a> {
    pub(super) device: &'a LanPairingDeviceRef,
    pub(super) pairing_id: Option<&'a str>,
    pub(super) source: &'a LanCanonicalHouseholdDeviceSource,
    pub(super) evidence_sources: &'a [LanDiscoveryEvidenceSource],
    pub(super) hint_sources: &'a [LanDiscoveryEvidenceSource],
    pub(super) service_identity_probe_evidence: &'a [LanServiceIdentityProbeEvidence],
    pub(super) observed_at: &'a str,
    pub(super) mac_assessment: Option<&'a LanMacIdentityAssessment>,
}

pub(super) fn evidence_records_for(
    input: &EvidenceRecordsInput<'_>,
) -> Vec<LanDiscoveryEvidenceRecord> {
    let context = evidence_context_for(input.source, input.evidence_sources);
    let mut records = Vec::new();
    push_network_identity_evidence(
        &mut records,
        input.device,
        &context,
        input.evidence_sources,
        input.observed_at,
        input.mac_assessment,
    );
    push_strong_identity_evidence(
        &mut records,
        input.device,
        input.pairing_id,
        &context,
        input.observed_at,
    );
    push_weak_name_evidence(
        &mut records,
        input.device,
        input.evidence_sources,
        input.observed_at,
    );
    push_vendor_evidence(
        &mut records,
        input.device,
        &context,
        input.observed_at,
        input.mac_assessment,
    );
    push_agent_evidence(&mut records, input.device, input.observed_at);
    push_service_probe_evidence(
        &mut records,
        input.device,
        input.service_identity_probe_evidence,
        input.observed_at,
    );
    push_hint_evidence(
        &mut records,
        input.device,
        input.hint_sources,
        input.observed_at,
    );
    push_trusted_registry_evidence(&mut records, input.device, input.source, input.observed_at);
    push_router_evidence(&mut records, input.device, &context, input.observed_at);
    push_fallback_evidence(&mut records, input.device, input.observed_at);
    records
}
