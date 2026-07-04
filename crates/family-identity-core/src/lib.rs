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
pub mod session_lifecycle;
pub mod setup_lifecycle;
mod setup_lifecycle_validation;
