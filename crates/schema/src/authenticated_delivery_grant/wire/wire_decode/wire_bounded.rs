use serde::{
    de::{DeserializeSeed, Error as _, SeqAccess, Visitor},
    Deserializer,
};

use super::{GrantWireFieldKind, GrantWireValue};
use crate::authenticated_delivery_grant::{
    AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES,
};

pub(in crate::authenticated_delivery_grant::wire) struct BoundedWireValue<'a> {
    kind: GrantWireFieldKind,
    signed_wire_bytes: &'a mut usize,
}

impl<'a> BoundedWireValue<'a> {
    pub(in crate::authenticated_delivery_grant::wire) const fn new(
        kind: GrantWireFieldKind,
        signed_wire_bytes: &'a mut usize,
    ) -> Self {
        Self {
            kind,
            signed_wire_bytes,
        }
    }
}

impl<'de> DeserializeSeed<'de> for BoundedWireValue<'_> {
    type Value = GrantWireValue;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(BoundedWireValueVisitor {
            kind: self.kind,
            signed_wire_bytes: self.signed_wire_bytes,
        })
    }
}

struct BoundedWireValueVisitor<'a> {
    kind: GrantWireFieldKind,
    signed_wire_bytes: &'a mut usize,
}

impl<'de> Visitor<'de> for BoundedWireValueVisitor<'_> {
    type Value = GrantWireValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a bounded authenticated delivery grant field")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if self.kind != GrantWireFieldKind::SchemaVersion {
            return Err(E::custom(
                "authenticated delivery grant field has the wrong wire type",
            ));
        }
        u16::try_from(value)
            .map(GrantWireValue::SchemaVersion)
            .map_err(|_error| {
                E::custom("authenticated delivery grant schema version is out of range")
            })
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if self.kind != GrantWireFieldKind::DryRun {
            return Err(E::custom(
                "authenticated delivery grant field has the wrong wire type",
            ));
        }
        Ok(GrantWireValue::DryRun(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if self.kind != GrantWireFieldKind::String {
            return Err(E::custom(
                "authenticated delivery grant field has the wrong wire type",
            ));
        }
        bounded_string(value, self.signed_wire_bytes).map(GrantWireValue::String)
    }

    fn visit_seq<M>(self, sequence: M) -> Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        if self.kind != GrantWireFieldKind::Signature {
            return Err(M::Error::custom(
                "authenticated delivery grant field has the wrong wire type",
            ));
        }
        bounded_signature(sequence, self.signed_wire_bytes).map(GrantWireValue::Signature)
    }
}

fn bounded_string<E>(value: &str, total: &mut usize) -> Result<String, E>
where
    E: serde::de::Error,
{
    if value.len() > AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES {
        return Err(E::custom(
            "authenticated delivery grant field exceeds its byte limit",
        ));
    }
    update_signed_wire_bytes(value.len(), total)?;
    Ok(value.to_owned())
}

fn bounded_signature<'de, M>(mut sequence: M, total: &mut usize) -> Result<Vec<u8>, M::Error>
where
    M: SeqAccess<'de>,
{
    let mut signature = Vec::with_capacity(AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES);
    while let Some(byte) = sequence.next_element::<u8>()? {
        if signature.len() == AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES {
            return Err(M::Error::custom(
                "authenticated delivery grant signature exceeds its byte limit",
            ));
        }
        signature.push(byte);
    }
    update_signed_wire_bytes(signature.len(), total)?;
    Ok(signature)
}

fn update_signed_wire_bytes<E>(length: usize, total: &mut usize) -> Result<(), E>
where
    E: serde::de::Error,
{
    *total = total
        .checked_add(length)
        .ok_or_else(|| E::custom("authenticated delivery grant aggregate length overflow"))?;
    if *total > AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES {
        return Err(E::custom(
            "authenticated delivery grant signed wire fields exceed their byte limit",
        ));
    }
    Ok(())
}
