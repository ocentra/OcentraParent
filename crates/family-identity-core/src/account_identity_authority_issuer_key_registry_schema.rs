use rusqlite::Connection;

use super::AccountIdentityIssuerError;

#[path = "account_identity_authority_issuer_key_registry_schema_definitions.rs"]
mod definitions;
#[path = "account_identity_authority_issuer_key_registry_schema_integrity.rs"]
mod integrity;
#[path = "account_identity_authority_issuer_key_registry_schema_objects.rs"]
mod objects;

pub(crate) fn validate(connection: &Connection) -> Result<(), AccountIdentityIssuerError> {
    definitions::validate(connection)?;
    objects::validate(connection)?;
    integrity::validate(connection)
}
