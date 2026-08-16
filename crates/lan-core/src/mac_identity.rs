use ocentra_parent_agent_protocol::constants;

mod assessment;

use assessment::{
    accepted_mac_assessment, finalize_mac_disposition, mac_disposition,
    parse_compact_and_normalized, rejected_mac_assessment, vendor_name_for_oui_prefix,
};

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
    pub(super) raw: String,
    pub(super) normalized: Option<String>,
    pub(super) compact: Option<String>,
    pub(super) oui_prefix: Option<String>,
    pub(super) vendor: Option<&'static str>,
    pub(super) disposition: LanMacIdentityDisposition,
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
