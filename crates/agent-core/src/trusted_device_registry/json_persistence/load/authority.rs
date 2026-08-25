use std::{collections::BTreeMap, io};

use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use serde_json::Value;

use super::super::super::{
    signer_authority_types::LanTrustedDeviceSignerAnchor, TrustedDeviceRegistry,
};

pub(super) fn reject_untrusted_paired_entries(registry: &TrustedDeviceRegistry) -> io::Result<()> {
    if registry
        .entries
        .iter()
        .any(|entry| entry.trust_state == LanPairingTrustState::Paired)
    {
        return Err(io::Error::from(io::ErrorKind::InvalidData));
    }
    Ok(())
}

pub(super) fn reject_untrusted_signer_anchors(value: &Value, key: &str) -> io::Result<()> {
    let Some(anchors) = value.get(key) else {
        return Ok(());
    };
    let persisted =
        serde_json::from_value::<BTreeMap<String, LanTrustedDeviceSignerAnchor>>(anchors.clone())
            .map_err(|_error| io::Error::from(io::ErrorKind::InvalidData))?;
    if persisted.is_empty() {
        Ok(())
    } else {
        Err(io::Error::from(io::ErrorKind::InvalidData))
    }
}
