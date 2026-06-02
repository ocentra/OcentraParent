use ocentra_parent_agent_protocol::{
    constants::{
        v08_browser_domain_adapter_proof as browser_proof,
        v08_cross_platform_enforcement_capability_proof as cross_proof,
        v08_os_adapter_product_proof as os_proof,
    },
    V08BrowserDomainAdapterProofReadModel, V08CrossPlatformEnforcementCapabilityProofReadModel,
    V08OsAdapterProductProofReadModel,
};

pub(super) fn expect_cross(
    read_model: &V08CrossPlatformEnforcementCapabilityProofReadModel,
    proof_entry_id: &'static str,
) {
    read_model
        .entries
        .iter()
        .find(|entry| entry.proof_entry_id == proof_entry_id)
        .expect(cross_proof::READ_MODEL_ID);
}

pub(super) fn expect_browser(
    read_model: &V08BrowserDomainAdapterProofReadModel,
    proof_entry_id: &'static str,
) {
    read_model
        .entries
        .iter()
        .find(|entry| entry.proof_entry_id == proof_entry_id)
        .expect(browser_proof::READ_MODEL_ID);
}

pub(super) fn expect_os(
    read_model: &V08OsAdapterProductProofReadModel,
    proof_entry_id: &'static str,
) {
    read_model
        .entries
        .iter()
        .find(|entry| entry.proof_entry_id == proof_entry_id)
        .expect(os_proof::READ_MODEL_ID);
}
