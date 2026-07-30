use serde::{de::DeserializeSeed, Deserialize, Deserializer};
use serde_json::value::RawValue;

use super::{GrantWireFieldKind, GrantWireValue};
use crate::authenticated_delivery_grant::{
    AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES,
};

const AUTHENTICATED_DELIVERY_GRANT_MAX_ENCODED_FIELD_BYTES: usize =
    AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES * 6 + 2;

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
        let raw = Box::<RawValue>::deserialize(deserializer)?;
        if raw.get().len() > AUTHENTICATED_DELIVERY_GRANT_MAX_ENCODED_FIELD_BYTES {
            return Err(serde::de::Error::custom(
                "authenticated delivery grant encoded field exceeds its byte limit",
            ));
        }
        decode_raw_value(raw.get(), self.kind, self.signed_wire_bytes)
    }
}

fn decode_raw_value<E>(
    raw: &str,
    kind: GrantWireFieldKind,
    signed_wire_bytes: &mut usize,
) -> Result<GrantWireValue, E>
where
    E: serde::de::Error,
{
    match kind {
        GrantWireFieldKind::SchemaVersion => {
            let value: u16 = serde_json::from_str::<u64>(raw)
                .map_err(|_error| {
                    E::custom("authenticated delivery grant field has the wrong wire type")
                })?
                .try_into()
                .map_err(|_error| {
                    E::custom("authenticated delivery grant schema version is out of range")
                })?;
            update_signing_wire_bytes(value.to_string().len(), signed_wire_bytes)?;
            Ok(GrantWireValue::SchemaVersion(value))
        }
        GrantWireFieldKind::PayloadLength => {
            let value = serde_json::from_str::<usize>(raw).map_err(|_error| {
                E::custom("authenticated delivery grant field has the wrong wire type")
            })?;
            update_signing_wire_bytes(value.to_string().len(), signed_wire_bytes)?;
            Ok(GrantWireValue::PayloadLength(value))
        }
        GrantWireFieldKind::DryRun => {
            let value = serde_json::from_str::<bool>(raw).map_err(|_error| {
                E::custom("authenticated delivery grant field has the wrong wire type")
            })?;
            update_signing_wire_bytes(value.to_string().len(), signed_wire_bytes)?;
            Ok(GrantWireValue::DryRun(value))
        }
        GrantWireFieldKind::String => serde_json::from_str::<String>(raw)
            .map_err(|_error| {
                E::custom("authenticated delivery grant field has the wrong wire type")
            })
            .and_then(|value| {
                bounded_string(&value, signed_wire_bytes)?;
                Ok(GrantWireValue::String(value))
            }),
        GrantWireFieldKind::Signature => serde_json::from_str::<Vec<u8>>(raw)
            .map_err(|_error| {
                E::custom("authenticated delivery grant field has the wrong wire type")
            })
            .and_then(|signature| bounded_signature(signature).map(GrantWireValue::Signature)),
    }
}

fn bounded_string<E>(value: &str, total: &mut usize) -> Result<(), E>
where
    E: serde::de::Error,
{
    if value.len() > AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES {
        return Err(E::custom(
            "authenticated delivery grant field exceeds its byte limit",
        ));
    }
    update_signing_wire_bytes(value.len(), total)?;
    Ok(())
}

fn bounded_signature<E>(signature: Vec<u8>) -> Result<Vec<u8>, E>
where
    E: serde::de::Error,
{
    if signature.len() > AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES {
        return Err(E::custom(
            "authenticated delivery grant signature exceeds its byte limit",
        ));
    }
    Ok(signature)
}

fn update_signing_wire_bytes<E>(value_length: usize, total: &mut usize) -> Result<(), E>
where
    E: serde::de::Error,
{
    *total = total
        .checked_add(std::mem::size_of::<u64>())
        .and_then(|total| total.checked_add(value_length))
        .ok_or_else(|| E::custom("authenticated delivery grant aggregate length overflow"))?;
    if *total > AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES {
        return Err(E::custom(
            "authenticated delivery grant signed wire fields exceed their byte limit",
        ));
    }
    Ok(())
}
