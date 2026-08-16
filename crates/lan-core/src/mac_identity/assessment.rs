use ocentra_parent_agent_protocol::constants;

use super::{LanMacIdentityAssessment, LanMacIdentityDisposition};

pub(super) fn parse_compact_and_normalized(raw: &str) -> Option<(String, String)> {
    let mut compact = String::with_capacity(12);
    for character in raw.chars() {
        if character.is_ascii_hexdigit() {
            compact.push(character.to_ascii_lowercase());
        } else if !matches!(character, ':' | '-' | '.') {
            return None;
        }
    }
    if compact.len() != 12 {
        return None;
    }
    let normalized = compact
        .as_bytes()
        .chunks(2)
        .map(|chunk| std::str::from_utf8(chunk).ok())
        .collect::<Option<Vec<_>>>()?
        .join(constants::lan_pairing::MAC_DASH);
    Some((compact, normalized))
}

pub(super) fn rejected_mac_assessment(
    raw: &str,
    disposition: LanMacIdentityDisposition,
) -> LanMacIdentityAssessment {
    LanMacIdentityAssessment {
        raw: raw.to_string(),
        normalized: None,
        compact: None,
        oui_prefix: None,
        vendor: None,
        disposition,
    }
}

pub(super) fn accepted_mac_assessment(
    raw: &str,
    normalized: String,
    compact: String,
    oui_prefix: Option<String>,
    vendor: Option<&'static str>,
    disposition: LanMacIdentityDisposition,
) -> LanMacIdentityAssessment {
    LanMacIdentityAssessment {
        raw: raw.to_string(),
        normalized: Some(normalized),
        compact: Some(compact),
        oui_prefix,
        vendor,
        disposition,
    }
}

pub(super) fn mac_disposition(compact: &str) -> Option<LanMacIdentityDisposition> {
    let first_octet = first_octet(compact)?;
    if compact == constants::lan_pairing::MAC_ZERO_COMPACT {
        return Some(LanMacIdentityDisposition::RejectedMalformed);
    }
    if compact == constants::lan_pairing::MAC_BROADCAST_COMPACT || is_multicast_octet(first_octet) {
        return Some(LanMacIdentityDisposition::RejectedMulticast);
    }
    if is_locally_administered_octet(first_octet) {
        return Some(LanMacIdentityDisposition::LocallyAdministered);
    }
    Some(LanMacIdentityDisposition::UnknownVendor)
}

pub(super) fn finalize_mac_disposition(
    disposition: LanMacIdentityDisposition,
    vendor: Option<&'static str>,
) -> LanMacIdentityDisposition {
    match disposition {
        LanMacIdentityDisposition::UnknownVendor if vendor.is_some() => {
            LanMacIdentityDisposition::KnownVendor
        }
        other => other,
    }
}

pub(super) fn vendor_name_for_oui_prefix(oui_prefix: &str) -> Option<&'static str> {
    KNOWN_OUI_VENDORS
        .iter()
        .find(|(prefix, _)| prefix.eq_ignore_ascii_case(oui_prefix))
        .map(|(_, vendor)| *vendor)
}

fn first_octet(compact: &str) -> Option<u8> {
    compact
        .get(0..2)
        .and_then(|value| u8::from_str_radix(value, 16).ok())
}

fn is_multicast_octet(first_octet: u8) -> bool {
    (first_octet & 0x01) == 0x01
}

fn is_locally_administered_octet(first_octet: u8) -> bool {
    (first_octet & 0x02) == 0x02
}

const KNOWN_OUI_VENDORS: [(&str, &str); 4] = [
    ("005056", "VMware, Inc."),
    ("54271e", "AzureWave Technology Inc."),
    ("70f8ae", "Microsoft Corporation"),
    ("dca632", "Raspberry Pi Trading Ltd"),
];
