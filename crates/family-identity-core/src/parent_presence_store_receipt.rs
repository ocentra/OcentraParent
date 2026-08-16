use std::fmt::Write;

use ocentra_eventing::ids::EventId;
use rusqlite::{params, Transaction};

use crate::parent_presence_store::ParentPresenceStoreError;

pub(crate) fn generate_opaque_receipt_ref() -> Result<String, ParentPresenceStoreError> {
    random_hex_identifier("parent-presence-receipt")
}

pub(crate) fn generate_opaque_decision_id() -> Result<EventId, ParentPresenceStoreError> {
    EventId::parse(random_hex_identifier("parent-presence-decision")?)
        .map_err(|_error| ParentPresenceStoreError::Unavailable)
}

pub(crate) fn is_valid_opaque_receipt_ref(value: &str) -> bool {
    let Some(entropy) = value.strip_prefix("parent-presence-receipt:") else {
        return false;
    };
    entropy.len() == 64
        && entropy
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

pub(crate) fn verify_consumed_receipt(
    transaction: &Transaction<'_>,
    challenge_ref: &str,
) -> Result<(), ParentPresenceStoreError> {
    let (receipt_count, receipt_ref) = transaction
        .query_row(
            "SELECT COUNT(*), MIN(receipt_ref) FROM parent_presence_receipts WHERE challenge_ref = ?1",
            params![challenge_ref],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, Option<String>>(1)?)),
        )
        .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    if receipt_count == 1
        && receipt_ref
            .as_deref()
            .is_some_and(is_valid_opaque_receipt_ref)
    {
        Ok(())
    } else {
        Err(ParentPresenceStoreError::IntegrityRejected)
    }
}

fn random_hex_identifier(prefix: &str) -> Result<String, ParentPresenceStoreError> {
    let mut entropy = [0_u8; 32];
    getrandom::fill(&mut entropy).map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    let mut encoded = String::with_capacity(64);
    for byte in entropy {
        write!(&mut encoded, "{byte:02x}")
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    }
    Ok(format!("{prefix}:{encoded}"))
}
