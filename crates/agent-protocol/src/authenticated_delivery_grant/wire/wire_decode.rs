use crate::authenticated_delivery_grant::AuthenticatedDeliveryGrant;

pub(super) mod wire_bounded;
pub(super) mod wire_fields;

use self::wire_fields::GrantWireFields;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum GrantWireFieldKind {
    SchemaVersion,
    String,
    DryRun,
    Signature,
}

pub(super) enum GrantWireValue {
    SchemaVersion(u16),
    String(String),
    DryRun(bool),
    Signature(Vec<u8>),
}

pub(super) fn build_grant(
    mut fields: GrantWireFields,
    names: &[&'static str],
) -> Result<AuthenticatedDeliveryGrant, String> {
    Ok(AuthenticatedDeliveryGrant {
        schema_version: schema_version(fields.required(0, names)?, names[0])?,
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
        dry_run: dry_run(fields.required(13, names)?, names[13])?,
        nonce: string(fields.required(14, names)?, names[14])?,
        issued_at: string(fields.required(15, names)?, names[15])?,
        expires_at: string(fields.required(16, names)?, names[16])?,
        revocation_version: string(fields.required(17, names)?, names[17])?,
        signature: signature(fields.required(18, names)?, names[18])?,
    })
}

fn schema_version(value: GrantWireValue, field: &str) -> Result<u16, String> {
    match value {
        GrantWireValue::SchemaVersion(value) => Ok(value),
        _ => Err(format!("field `{field}` has the wrong wire type")),
    }
}

fn string(value: GrantWireValue, field: &str) -> Result<String, String> {
    match value {
        GrantWireValue::String(value) => Ok(value),
        _ => Err(format!("field `{field}` has the wrong wire type")),
    }
}

fn dry_run(value: GrantWireValue, field: &str) -> Result<bool, String> {
    match value {
        GrantWireValue::DryRun(value) => Ok(value),
        _ => Err(format!("field `{field}` has the wrong wire type")),
    }
}

fn signature(value: GrantWireValue, field: &str) -> Result<Vec<u8>, String> {
    match value {
        GrantWireValue::Signature(value) => Ok(value),
        _ => Err(format!("field `{field}` has the wrong wire type")),
    }
}
