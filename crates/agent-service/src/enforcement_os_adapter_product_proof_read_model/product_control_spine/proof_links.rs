use ocentra_parent_agent_protocol::constants::v08_browser_domain_adapter_proof as browser_proof;
use ocentra_parent_agent_protocol::constants::v08_cross_platform_enforcement_capability_proof as cross_proof;
use ocentra_parent_agent_protocol::constants::v08_os_adapter_product_proof as os_proof;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofReadModel;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilityProofReadModel;
use ocentra_parent_agent_protocol::enforcement_os_adapter_product_proof::V08OsAdapterProductProofReadModel;

pub(super) fn expect_cross(
    read_model: &V08CrossPlatformEnforcementCapabilityProofReadModel,
    proof_entry_id: &'static str,
) {
    if !read_model
        .entries
        .iter()
        .any(|entry| entry.proof_entry_id == proof_entry_id)
    {
        panic!("{}", cross_proof::READ_MODEL_ID);
    }
}

pub(super) fn expect_browser(
    read_model: &V08BrowserDomainAdapterProofReadModel,
    proof_entry_id: &'static str,
) {
    if !read_model
        .entries
        .iter()
        .any(|entry| entry.proof_entry_id == proof_entry_id)
    {
        panic!("{}", browser_proof::READ_MODEL_ID);
    }
}

pub(super) fn expect_os(
    read_model: &V08OsAdapterProductProofReadModel,
    proof_entry_id: &'static str,
) {
    if !read_model
        .entries
        .iter()
        .any(|entry| entry.proof_entry_id == proof_entry_id)
    {
        panic!("{}", os_proof::READ_MODEL_ID);
    }
}
