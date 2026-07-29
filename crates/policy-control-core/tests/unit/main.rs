type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[macro_use]
#[path = "../support/mod.rs"]
mod test_support;

#[path = "policy_control.rs"]
mod policy_control;

#[path = "authenticated_delivery_grant.rs"]
mod authenticated_delivery_grant;
#[path = "authenticated_delivery_grant_provenance.rs"]
mod authenticated_delivery_grant_provenance;
#[path = "policy_authority.rs"]
mod policy_authority;

#[path = "policy_contract_helpers.rs"]
mod policy_contract_helpers;

#[path = "policy_conflict.rs"]
mod policy_conflict;

#[path = "policy_compiler.rs"]
mod policy_compiler;

#[path = "policy_delivery_helpers.rs"]
mod policy_delivery_helpers;

#[path = "policy_delivery.rs"]
mod policy_delivery;

#[path = "policy_delivery_receipt_helpers.rs"]
mod policy_delivery_receipt_helpers;

#[path = "policy_delivery_receipt_identity.rs"]
mod policy_delivery_receipt_identity;

#[path = "policy_delivery_receipt_redaction.rs"]
mod policy_delivery_receipt_redaction;

#[path = "policy_delivery_record_boundaries.rs"]
mod policy_delivery_record_boundaries;

#[path = "policy_delivery_metadata.rs"]
mod policy_delivery_metadata;

#[path = "policy_delivery_receipt.rs"]
mod policy_delivery_receipt;

#[path = "policy_event.rs"]
mod policy_event;

#[path = "policy_preview.rs"]
mod policy_preview;

#[path = "policy_request.rs"]
mod policy_request;

#[path = "policy_source.rs"]
mod policy_source;
