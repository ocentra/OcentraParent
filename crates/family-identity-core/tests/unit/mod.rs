mod device_scope;
#[path = "household_authority.rs"]
mod household_authority_tests;
mod invite_recovery_repository;
mod invite_recovery_repository_schema;
mod invite_recovery_repository_security;
mod parent_step_up_authority;
mod parent_step_up_proof;
mod recovery_owner_ack_ops;
mod session_lifecycle;
mod setup_lifecycle;
mod trust_bootstrap;
mod trust_bootstrap_cross_process;
mod trust_bootstrap_event_delivery;
mod trust_bootstrap_expiry;
mod trust_bootstrap_nonce_process;
mod trust_bootstrap_probes;
mod trust_bootstrap_receipt_integrity;
mod trust_bootstrap_store_integrity;
mod trust_bootstrap_store_metadata;
mod trust_bootstrap_store_schema;
mod trust_bootstrap_store_security;

fn open_parent_presence_test_port(
    path: impl Into<std::path::PathBuf>,
) -> Result<
    ocentra_family_identity_core::parent_presence::ParentPresenceVerificationPort,
    ocentra_family_identity_core::parent_presence::ParentPresenceStorageFailureReason,
> {
    ocentra_family_identity_core::parent_presence::ParentPresenceVerificationPort::open_unsealed_test_custody(path)
}
