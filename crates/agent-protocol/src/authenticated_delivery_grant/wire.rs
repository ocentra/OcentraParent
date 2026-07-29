use serde::{
    de::{Error as _, MapAccess, Visitor},
    Deserialize, Deserializer,
};

use self::wire_decode::wire_bounded::BoundedWireValue;
use self::wire_decode::wire_fields::GrantWireFields;
use self::wire_decode::{build_grant, GrantWireFieldKind};
use super::AuthenticatedDeliveryGrant;

mod wire_decode;

const GRANT_WIRE_FIELD_NAMES: &[&str] = &[
    "schemaVersion",
    "issuerKeyId",
    "issuerActorId",
    "householdId",
    "parentDeviceId",
    "childProfileId",
    "targetDeviceId",
    "policyDecisionId",
    "policyVersion",
    "actionId",
    "capabilityId",
    "evidenceDigest",
    "payloadDigest",
    "dryRun",
    "nonce",
    "issuedAt",
    "expiresAt",
    "revocationVersion",
    "signature",
];

const GRANT_WIRE_FIELD_KINDS: &[GrantWireFieldKind] = &[
    GrantWireFieldKind::SchemaVersion,
    GrantWireFieldKind::String,
    GrantWireFieldKind::String,
    GrantWireFieldKind::String,
    GrantWireFieldKind::String,
    GrantWireFieldKind::String,
    GrantWireFieldKind::String,
    GrantWireFieldKind::String,
    GrantWireFieldKind::String,
    GrantWireFieldKind::String,
    GrantWireFieldKind::String,
    GrantWireFieldKind::String,
    GrantWireFieldKind::String,
    GrantWireFieldKind::DryRun,
    GrantWireFieldKind::String,
    GrantWireFieldKind::String,
    GrantWireFieldKind::String,
    GrantWireFieldKind::String,
    GrantWireFieldKind::Signature,
];

#[derive(Clone, Copy)]
pub(super) struct GrantWireField(usize);

impl GrantWireField {
    fn kind(&self) -> GrantWireFieldKind {
        GRANT_WIRE_FIELD_KINDS[self.0]
    }
}

impl<'de> Deserialize<'de> for GrantWireField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(GrantWireFieldVisitor)
    }
}

struct GrantWireFieldVisitor;

impl Visitor<'_> for GrantWireFieldVisitor {
    type Value = GrantWireField;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("an authenticated delivery grant field")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        GRANT_WIRE_FIELD_NAMES
            .iter()
            .position(|name| *name == value)
            .map(GrantWireField)
            .ok_or_else(|| E::unknown_field(value, GRANT_WIRE_FIELD_NAMES))
    }
}

struct GrantWireVisitor;

impl<'de> Visitor<'de> for GrantWireVisitor {
    type Value = AuthenticatedDeliveryGrant;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a bounded authenticated delivery grant object")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut signed_wire_bytes = 0_usize;
        let mut fields = GrantWireFields::new();
        while let Some(field) = map.next_key::<GrantWireField>()? {
            let value =
                map.next_value_seed(BoundedWireValue::new(field.kind(), &mut signed_wire_bytes))?;
            fields
                .insert(field, value, GRANT_WIRE_FIELD_NAMES)
                .map_err(M::Error::custom)?;
        }
        build_grant(fields, GRANT_WIRE_FIELD_NAMES).map_err(M::Error::custom)
    }
}

impl<'de> Deserialize<'de> for AuthenticatedDeliveryGrant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "AuthenticatedDeliveryGrant",
            GRANT_WIRE_FIELD_NAMES,
            GrantWireVisitor,
        )
    }
}
