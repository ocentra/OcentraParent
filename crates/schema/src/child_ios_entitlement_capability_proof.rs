pub mod constants;
pub mod enums_capability;
pub mod enums_runtime;
pub mod identifiers;
pub mod lifecycle_proofs;
#[macro_use]
mod macros;
pub mod proof_types;
pub mod sample;
pub mod surface_proofs;

pub use constants::CHILD_IOS_ENTITLEMENT_CAPABILITY_PROOF_SCHEMA_VERSION;

pub fn sample_child_ios_entitlement_capability_read_model(
) -> ChildIosEntitlementCapabilityReadModel {
    sample::sample_child_ios_entitlement_capability_read_model()
}
