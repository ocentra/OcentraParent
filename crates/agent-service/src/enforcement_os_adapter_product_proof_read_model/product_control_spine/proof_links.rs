use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofReadModel;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilityProofEntry;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilityProofReadModel;
use ocentra_parent_agent_protocol::enforcement_os_adapter_product_proof::V08OsAdapterProductProofReadModel;

use super::ProofEntryId;

pub(super) fn expect_cross(
    read_model: &V08CrossPlatformEnforcementCapabilityProofReadModel,
    proof_entry_id: ProofEntryId,
) -> &V08CrossPlatformEnforcementCapabilityProofEntry {
    read_model
        .entries
        .iter()
        .find(|entry| entry.proof_entry_id == proof_entry_id.0)
        .expect_value(proof_entry_id.0)
}

pub(super) fn expect_browser(
    read_model: &V08BrowserDomainAdapterProofReadModel,
    proof_entry_id: ProofEntryId,
) {
    read_model
        .entries
        .iter()
        .find(|entry| entry.proof_entry_id == proof_entry_id.0)
        .expect_value(proof_entry_id.0);
}

pub(super) fn expect_os(
    read_model: &V08OsAdapterProductProofReadModel,
    proof_entry_id: ProofEntryId,
) {
    read_model
        .entries
        .iter()
        .find(|entry| entry.proof_entry_id == proof_entry_id.0)
        .expect_value(proof_entry_id.0);
}
