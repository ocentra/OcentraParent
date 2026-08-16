use crate::authenticated_delivery_grant::AuthenticatedDeliveryGrant;

pub(super) mod wire_bounded;
pub(super) mod wire_fields;
pub(super) mod wire_owned_values;
pub(super) mod wire_values;

use self::wire_fields::GrantWireFields;
use self::wire_owned_values::{signature, string};
use self::wire_values::{dry_run, payload_length, schema_version};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum GrantWireFieldKind {
    SchemaVersion,
    PayloadLength,
    String,
    DryRun,
    Signature,
}

pub(super) enum GrantWireValue {
    SchemaVersion(u16),
    PayloadLength(usize),
    String(String),
    DryRun(bool),
    Signature(Vec<u8>),
}

pub(super) fn build_grant(
    mut fields: GrantWireFields,
    names: &[&'static str],
) -> Result<AuthenticatedDeliveryGrant, String> {
    let schema_version_value = fields.required(0, names)?;
    let payload_length_value = fields.required(13, names)?;
    let dry_run_value = fields.required(14, names)?;
    Ok(AuthenticatedDeliveryGrant {
        schema_version: schema_version(&schema_version_value, names[0])?,
        issuer_key_id: string(fields.required(1, names)?, names[1])?,
        issuer_actor_id: string(fields.required(2, names)?, names[2])?,
        household_id: string(fields.required(3, names)?, names[3])?,
        parent_device_id: string(fields.required(4, names)?, names[4])?,
        child_profile_id: string(fields.required(5, names)?, names[5])?,
        target_device_id: string(fields.required(6, names)?, names[6])?,
        policy_decision_id: string(fields.required(7, names)?, names[7])?,
        policy_version: string(fields.required(8, names)?, names[8])?,
        action_id: string(fields.required(9, names)?, names[9])?,
        capability_id: string(fields.required(10, names)?, names[10])?,
        evidence_digest: string(fields.required(11, names)?, names[11])?,
        payload_digest: string(fields.required(12, names)?, names[12])?,
        payload_length: payload_length(&payload_length_value, names[13])?,
        dry_run: dry_run(&dry_run_value, names[14])?,
        nonce: string(fields.required(15, names)?, names[15])?,
        issued_at: string(fields.required(16, names)?, names[16])?,
        expires_at: string(fields.required(17, names)?, names[17])?,
        revocation_version: string(fields.required(18, names)?, names[18])?,
        signature: signature(fields.required(19, names)?, names[19])?,
    })
}
