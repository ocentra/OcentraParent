mod entry_factory;
mod proof_links;
mod surface_entries;

use ocentra_parent_agent_protocol::{
    constants::v08_enforcement_product_control_spine as spine, policy_constants,
    V08EnforcementProductControlSpineReadModel,
};

use crate::{
    enforcement_browser_domain_adapter_proof_read_model::v08_browser_domain_adapter_proof_read_model,
    enforcement_cross_platform_capability_proof_read_model::v08_cross_platform_enforcement_capability_proof_read_model,
};

pub(crate) fn v08_enforcement_product_control_spine_read_model(
    generated_at: &str,
) -> V08EnforcementProductControlSpineReadModel {
    let cross_platform = v08_cross_platform_enforcement_capability_proof_read_model(generated_at);
    let browser_domain = v08_browser_domain_adapter_proof_read_model(generated_at);
    let os_product = super::v08_os_adapter_product_proof_read_model(generated_at);

    V08EnforcementProductControlSpineReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: spine::READ_MODEL_ID.to_string(),
        generated_at: generated_at.to_string(),
        source_read_model_ids: vec![
            spine::SOURCE_CROSS_PLATFORM_CAPABILITY.to_string(),
            spine::SOURCE_BROWSER_DOMAIN.to_string(),
            spine::SOURCE_OS_ADAPTER_PRODUCT.to_string(),
            spine::SOURCE_BROWSER_POLICY_PREVIEW.to_string(),
        ],
        entries: surface_entries::entries(
            &cross_platform,
            &browser_domain,
            &os_product,
            generated_at,
        ),
    }
}
