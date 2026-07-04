use ocentra_parent_agent_protocol::constants;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LanMacIdentityDisposition {
    KnownVendor,
    UnknownVendor,
    LocallyAdministered,
    RejectedMulticast,
    RejectedMalformed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanMacIdentityAssessment {
    raw: String,
    normalized: Option<String>,
    compact: Option<String>,
    oui_prefix: Option<String>,
    vendor: Option<&'static str>,
    disposition: LanMacIdentityDisposition,
}

impl LanMacIdentityAssessment {
    pub fn normalized(&self) -> Option<&str> {
        self.normalized.as_deref()
    }

    pub fn normalized_owned(&self) -> Option<String> {
        self.normalized.clone()
    }

    pub fn vendor_name(&self) -> Option<&'static str> {
        self.vendor
    }

    pub fn disposition(&self) -> LanMacIdentityDisposition {
        self.disposition
    }

    pub fn identity_key_allowed(&self) -> bool {
        !matches!(
            self.disposition,
            LanMacIdentityDisposition::RejectedMalformed
                | LanMacIdentityDisposition::RejectedMulticast
        )
    }

    pub fn stable_identity_key_allowed(&self) -> bool {
        matches!(
            self.disposition,
            LanMacIdentityDisposition::KnownVendor | LanMacIdentityDisposition::UnknownVendor
        )
    }

    pub fn vendor_evidence_value(&self) -> String {
        match self.disposition {
            LanMacIdentityDisposition::KnownVendor => self
                .vendor
                .unwrap_or(constants::value::UNKNOWN_HOST)
                .to_string(),
            LanMacIdentityDisposition::UnknownVendor => {
                self.oui_prefix.clone().unwrap_or_else(|| self.raw.clone())
            }
            LanMacIdentityDisposition::LocallyAdministered
            | LanMacIdentityDisposition::RejectedMulticast => {
                self.normalized.clone().unwrap_or_else(|| self.raw.clone())
            }
            LanMacIdentityDisposition::RejectedMalformed => self.raw.clone(),
        }
    }

    pub fn vendor_evidence_note(&self) -> Option<&'static str> {
        match self.disposition {
            LanMacIdentityDisposition::KnownVendor => None,
            LanMacIdentityDisposition::UnknownVendor => {
                Some(constants::lan_pairing::LAN_VENDOR_UNKNOWN_PREFIX_NOTE)
            }
            LanMacIdentityDisposition::LocallyAdministered => {
                Some(constants::lan_pairing::LAN_VENDOR_LOCAL_ADMINISTERED_NOTE)
            }
            LanMacIdentityDisposition::RejectedMulticast => {
                Some(constants::lan_pairing::LAN_VENDOR_REJECT_MULTICAST_NOTE)
            }
            LanMacIdentityDisposition::RejectedMalformed => {
                Some(constants::lan_pairing::LAN_VENDOR_REJECT_MALFORMED_NOTE)
            }
        }
    }
}

pub fn assess_mac_address(value: Option<&str>) -> Option<LanMacIdentityAssessment> {
    let raw = value?.trim();
    if raw.is_empty() {
        return None;
    }

    let Some((compact, normalized)) = parse_compact_and_normalized(raw) else {
        return Some(rejected_mac_assessment(
            raw,
            LanMacIdentityDisposition::RejectedMalformed,
        ));
    };

    let oui_prefix = compact.get(0..6).map(str::to_string);
    let disposition = mac_disposition(&compact)?;
    let vendor = oui_prefix.as_deref().and_then(vendor_name_for_oui_prefix);
    Some(accepted_mac_assessment(
        raw,
        normalized,
        compact,
        oui_prefix,
        vendor,
        finalize_mac_disposition(disposition, vendor),
    ))
}

pub fn normalize_scan_mac_address(value: &str) -> Option<String> {
    let assessment = assess_mac_address(Some(value))?;
    assessment
        .identity_key_allowed()
        .then_some(assessment.normalized_owned())
        .flatten()
}

fn parse_compact_and_normalized(raw: &str) -> Option<(String, String)> {
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

fn rejected_mac_assessment(
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

fn accepted_mac_assessment(
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

fn mac_disposition(compact: &str) -> Option<LanMacIdentityDisposition> {
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

fn finalize_mac_disposition(
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

fn vendor_name_for_oui_prefix(oui_prefix: &str) -> Option<&'static str> {
    KNOWN_OUI_VENDORS
        .iter()
        .find(|(prefix, _)| prefix.eq_ignore_ascii_case(oui_prefix))
        .map(|(_, vendor)| *vendor)
}

const KNOWN_OUI_VENDORS: [(&str, &str); 4] = [
    ("005056", "VMware, Inc."),
    ("54271e", "AzureWave Technology Inc."),
    ("70f8ae", "Microsoft Corporation"),
    ("dca632", "Raspberry Pi Trading Ltd"),
];
