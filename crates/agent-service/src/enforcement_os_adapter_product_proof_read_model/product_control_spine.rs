#[path = "product_control_spine/entry_factory.rs"]
mod entry_factory;
#[path = "product_control_spine/proof_links.rs"]
mod proof_links;
#[path = "product_control_spine/surface_entries.rs"]
mod surface_entries;

use ocentra_parent_agent_protocol::constants::v08_enforcement_product_control_spine as spine;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlSpineReadModel;
use ocentra_parent_agent_protocol::policy_constants;

use crate::{
    enforcement_browser_domain_adapter_proof_read_model::v08_browser_domain_adapter_proof_read_model,
    enforcement_cross_platform_capability_proof_read_model::v08_cross_platform_enforcement_capability_proof_read_model,
};

#[derive(Clone, Debug)]
pub(crate) struct GeneratedAtText(pub(crate) String);

impl<T> From<T> for GeneratedAtText
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ProofEntryId(pub(crate) &'static str);

pub(crate) fn v08_enforcement_product_control_spine_read_model(
    generated_at: impl Into<GeneratedAtText>,
) -> V08EnforcementProductControlSpineReadModel {
    let generated_at = generated_at.into();
    let generated_at_value = generated_at.0.clone();
    let cross_platform = v08_cross_platform_enforcement_capability_proof_read_model(
        crate::enforcement_cross_platform_capability_proof_read_model::GeneratedAtTextRef(
            generated_at.0.as_str(),
        ),
    );
    let browser_domain = v08_browser_domain_adapter_proof_read_model(generated_at.clone());
    let os_product = super::v08_os_adapter_product_proof_read_model(super::GeneratedAtTextRef(
        generated_at.0.as_str(),
    ));

    V08EnforcementProductControlSpineReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: spine::READ_MODEL_ID.to_string(),
        generated_at: generated_at_value,
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
            &generated_at,
        ),
    }
}
