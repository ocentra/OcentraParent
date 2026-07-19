#![forbid(unsafe_code)]

#[macro_use]
mod family_identity_text_ids;
pub mod family_identity;
mod family_identity_contract_text;
mod family_identity_helpers;
mod family_identity_profiles;
mod family_identity_profiles_validation;
mod family_identity_setup_records;
pub mod household_authority;
mod household_authority_validation;
pub mod parent_presence;
mod parent_presence_port;
mod parent_presence_store;
mod parent_presence_store_integrity;
mod parent_presence_store_path;
mod parent_presence_store_receipt;
mod parent_presence_store_schema;
pub mod session_lifecycle;
pub mod setup_lifecycle;
mod setup_lifecycle_validation;
pub mod trust_bootstrap;
mod trust_bootstrap_clock;
mod trust_bootstrap_time;
mod trust_bootstrap_validation;
mod trust_bootstrap_validation_map;
