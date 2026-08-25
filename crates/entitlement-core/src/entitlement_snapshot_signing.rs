#![forbid(unsafe_code)]

use super::{SignedEntitlementSnapshot, ENTITLEMENT_SNAPSHOT_SIGNING_DOMAIN};

impl SignedEntitlementSnapshot {
    /// Returns the exact issuer-signed bytes. The signature itself is never
    /// included, so a verifier can independently authenticate the envelope.
    pub(crate) fn signing_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(512);
        bytes.extend_from_slice(ENTITLEMENT_SNAPSHOT_SIGNING_DOMAIN);
        append_u16(&mut bytes, self.schema_version);
        append_text(&mut bytes, self.snapshot_id.as_str());
        append_text(&mut bytes, self.account_ref.as_str());
        append_text(&mut bytes, self.household_ref.as_str());
        append_text(&mut bytes, self.trusted_device_ref.as_str());
        append_text(
            &mut bytes,
            super::wire_names::plan_tier_wire_name(self.plan_tier),
        );
        append_u32(&mut bytes, self.feature_flags.len() as u32);
        for flag in &self.feature_flags {
            append_text(
                &mut bytes,
                super::capability_wire_names::capability_wire_name(flag.capability),
            );
            append_bool(&mut bytes, flag.enabled);
        }
        append_u32(&mut bytes, self.limits.child_device_limit);
        append_u32(&mut bytes, self.base_child_device_limit);
        append_u32(&mut bytes, self.active_referral_credits);
        append_u32(&mut bytes, self.paid_extra_child_device_seats);
        append_u32(&mut bytes, self.effective_child_device_limit);
        append_text(&mut bytes, &self.issued_at);
        append_text(&mut bytes, &self.expires_at);
        append_optional_text(&mut bytes, self.grace_until.as_deref());
        append_bool(&mut bytes, self.livemode);
        append_text(&mut bytes, self.revocation_cursor.as_str());
        append_u64(&mut bytes, self.authority_generation);
        append_bool(&mut bytes, self.device_trust_required);
        append_text(&mut bytes, self.package_build_ref.as_str());
        append_text(
            &mut bytes,
            super::wire_names::release_channel_wire_name(self.release_channel),
        );
        append_text(&mut bytes, self.signature_key_id.as_str());
        bytes
    }
}

fn append_text(bytes: &mut Vec<u8>, value: &str) {
    append_u64(bytes, value.len() as u64);
    bytes.extend_from_slice(value.as_bytes());
}

fn append_optional_text(bytes: &mut Vec<u8>, value: Option<&str>) {
    match value {
        Some(value) => {
            bytes.push(1);
            append_text(bytes, value);
        }
        None => bytes.push(0),
    }
}

fn append_bool(bytes: &mut Vec<u8>, value: bool) {
    bytes.push(u8::from(value));
}

fn append_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn append_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn append_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_be_bytes());
}
