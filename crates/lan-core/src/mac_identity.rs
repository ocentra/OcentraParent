use ocentra_parent_agent_protocol::constants;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LanMacIdentityDisposition {
    KnownVendor,
    UnknownVendor,
    LocallyAdministered,
    RejectedMulticast,
    RejectedMalformed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LanMacIdentityAssessment {
    raw: String,
    normalized: Option<String>,
    compact: Option<String>,
    oui_prefix: Option<String>,
    vendor: Option<&'static str>,
    disposition: LanMacIdentityDisposition,
}

impl LanMacIdentityAssessment {
    pub(crate) fn normalized(&self) -> Option<&str> {
        self.normalized.as_deref()
    }

    pub(crate) fn normalized_owned(&self) -> Option<String> {
        self.normalized.clone()
    }

    pub(crate) fn vendor_name(&self) -> Option<&'static str> {
        self.vendor
    }

    pub(crate) fn disposition(&self) -> LanMacIdentityDisposition {
        self.disposition
    }

    pub(crate) fn identity_key_allowed(&self) -> bool {
        !matches!(
            self.disposition,
            LanMacIdentityDisposition::RejectedMalformed
                | LanMacIdentityDisposition::RejectedMulticast
        )
    }

    pub(crate) fn vendor_evidence_value(&self) -> String {
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

    pub(crate) fn vendor_evidence_note(&self) -> Option<&'static str> {
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

pub(crate) fn assess_mac_address(value: Option<&str>) -> Option<LanMacIdentityAssessment> {
    let raw = value?.trim();
    if raw.is_empty() {
        return None;
    }

    let Some((compact, normalized)) = parse_compact_and_normalized(raw) else {
        return Some(LanMacIdentityAssessment {
            raw: raw.to_string(),
            normalized: None,
            compact: None,
            oui_prefix: None,
            vendor: None,
            disposition: LanMacIdentityDisposition::RejectedMalformed,
        });
    };

    let Some(first_octet) = compact
        .get(0..2)
        .and_then(|value| u8::from_str_radix(value, 16).ok())
    else {
        return Some(LanMacIdentityAssessment {
            raw: raw.to_string(),
            normalized: None,
            compact: None,
            oui_prefix: None,
            vendor: None,
            disposition: LanMacIdentityDisposition::RejectedMalformed,
        });
    };

    if compact == constants::lan_pairing::MAC_ZERO_COMPACT {
        return Some(LanMacIdentityAssessment {
            raw: raw.to_string(),
            normalized: Some(normalized),
            compact: Some(compact),
            oui_prefix: None,
            vendor: None,
            disposition: LanMacIdentityDisposition::RejectedMalformed,
        });
    }

    if compact == constants::lan_pairing::MAC_BROADCAST_COMPACT || (first_octet & 0x01) == 0x01 {
        let oui_prefix = compact.get(0..6).map(str::to_string);
        return Some(LanMacIdentityAssessment {
            raw: raw.to_string(),
            normalized: Some(normalized),
            compact: Some(compact),
            oui_prefix,
            vendor: None,
            disposition: LanMacIdentityDisposition::RejectedMulticast,
        });
    }

    let oui_prefix = compact.get(0..6).map(str::to_string);
    if (first_octet & 0x02) == 0x02 {
        return Some(LanMacIdentityAssessment {
            raw: raw.to_string(),
            normalized: Some(normalized),
            compact: Some(compact),
            oui_prefix,
            vendor: None,
            disposition: LanMacIdentityDisposition::LocallyAdministered,
        });
    }

    let vendor = oui_prefix.as_deref().and_then(vendor_name_for_oui_prefix);
    Some(LanMacIdentityAssessment {
        raw: raw.to_string(),
        normalized: Some(normalized),
        compact: Some(compact),
        oui_prefix,
        vendor,
        disposition: if vendor.is_some() {
            LanMacIdentityDisposition::KnownVendor
        } else {
            LanMacIdentityDisposition::UnknownVendor
        },
    })
}

pub(crate) fn normalize_scan_mac_address(value: &str) -> Option<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_vendor_lookup_normalizes_mac_and_returns_vendor() {
        let assessment = assess_mac_address(Some("54:27:1E:97:C3:31"))
            .unwrap_or_else(|| unreachable!("assessment exists"));

        assert_eq!(
            assessment.normalized(),
            Some(constants::lan_pairing::TEST_LAN_MAC)
        );
        assert_eq!(assessment.vendor_name(), Some("AzureWave Technology Inc."));
        assert_eq!(
            assessment.disposition(),
            LanMacIdentityDisposition::KnownVendor
        );
        assert!(assessment.identity_key_allowed());
    }

    #[test]
    fn unknown_vendor_prefix_stays_identity_eligible() {
        let assessment = assess_mac_address(Some(constants::lan_pairing::TEST_ROUTER_MAC))
            .unwrap_or_else(|| unreachable!("assessment exists"));

        assert_eq!(
            assessment.normalized(),
            Some(constants::lan_pairing::TEST_ROUTER_MAC)
        );
        assert_eq!(assessment.vendor_name(), None);
        assert_eq!(
            assessment.disposition(),
            LanMacIdentityDisposition::UnknownVendor
        );
        assert_eq!(
            assessment.vendor_evidence_note(),
            Some(constants::lan_pairing::LAN_VENDOR_UNKNOWN_PREFIX_NOTE)
        );
        assert!(assessment.identity_key_allowed());
    }

    #[test]
    fn locally_administered_mac_stays_visible_but_warned() {
        let assessment = assess_mac_address(Some("02-aa-bb-cc-dd-ee"))
            .unwrap_or_else(|| unreachable!("assessment exists"));

        assert_eq!(assessment.normalized(), Some("02-aa-bb-cc-dd-ee"));
        assert_eq!(
            assessment.disposition(),
            LanMacIdentityDisposition::LocallyAdministered
        );
        assert!(assessment.identity_key_allowed());
        assert_eq!(
            assessment.vendor_evidence_note(),
            Some(constants::lan_pairing::LAN_VENDOR_LOCAL_ADMINISTERED_NOTE)
        );
    }

    #[test]
    fn multicast_mac_is_rejected_for_scan_identity() {
        let assessment = assess_mac_address(Some("01:00:5e:00:00:fb"))
            .unwrap_or_else(|| unreachable!("assessment exists"));

        assert_eq!(assessment.normalized(), Some("01-00-5e-00-00-fb"));
        assert_eq!(
            assessment.disposition(),
            LanMacIdentityDisposition::RejectedMulticast
        );
        assert!(!assessment.identity_key_allowed());
        assert!(normalize_scan_mac_address("01:00:5e:00:00:fb").is_none());
    }

    #[test]
    fn malformed_mac_is_rejected() {
        let assessment = assess_mac_address(Some("zz-not-a-mac"))
            .unwrap_or_else(|| unreachable!("assessment exists"));

        assert_eq!(assessment.normalized(), None);
        assert_eq!(
            assessment.disposition(),
            LanMacIdentityDisposition::RejectedMalformed
        );
        assert!(!assessment.identity_key_allowed());
    }
}
