use ocentra_lan_core::mac_identity::{
    assess_mac_address, normalize_scan_mac_address, LanMacIdentityDisposition,
};
use ocentra_parent_agent_protocol::constants;

#[test]
fn known_vendor_lookup_normalizes_mac_and_returns_vendor() {
    let assessment = assess_mac_address(Some("54:27:1E:97:C3:31"));

    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.normalized()),
        Some(Some(constants::lan_pairing::TEST_LAN_MAC))
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.vendor_name()),
        Some(Some("AzureWave Technology Inc."))
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.disposition()),
        Some(LanMacIdentityDisposition::KnownVendor)
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.identity_key_allowed()),
        Some(true)
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.stable_identity_key_allowed()),
        Some(true)
    );
}

#[test]
fn unknown_vendor_prefix_stays_identity_eligible() {
    let assessment = assess_mac_address(Some(constants::lan_pairing::TEST_ROUTER_MAC));

    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.normalized()),
        Some(Some(constants::lan_pairing::TEST_ROUTER_MAC))
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.vendor_name()),
        Some(None)
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.disposition()),
        Some(LanMacIdentityDisposition::UnknownVendor)
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.vendor_evidence_note()),
        Some(Some(constants::lan_pairing::LAN_VENDOR_UNKNOWN_PREFIX_NOTE))
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.identity_key_allowed()),
        Some(true)
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.stable_identity_key_allowed()),
        Some(true)
    );
}

#[test]
fn locally_administered_mac_stays_visible_but_warned() {
    let assessment = assess_mac_address(Some("02-aa-bb-cc-dd-ee"));

    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.normalized()),
        Some(Some("02-aa-bb-cc-dd-ee"))
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.disposition()),
        Some(LanMacIdentityDisposition::LocallyAdministered)
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.identity_key_allowed()),
        Some(true)
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.stable_identity_key_allowed()),
        Some(false)
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.vendor_evidence_note()),
        Some(Some(
            constants::lan_pairing::LAN_VENDOR_LOCAL_ADMINISTERED_NOTE
        ))
    );
}

#[test]
fn multicast_mac_is_rejected_for_scan_identity() {
    let assessment = assess_mac_address(Some("01:00:5e:00:00:fb"));

    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.normalized()),
        Some(Some("01-00-5e-00-00-fb"))
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.disposition()),
        Some(LanMacIdentityDisposition::RejectedMulticast)
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.identity_key_allowed()),
        Some(false)
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.stable_identity_key_allowed()),
        Some(false)
    );
    assert!(normalize_scan_mac_address("01:00:5e:00:00:fb").is_none());
}

#[test]
fn malformed_mac_is_rejected() {
    let assessment = assess_mac_address(Some("zz-not-a-mac"));

    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.normalized()),
        Some(None)
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.disposition()),
        Some(LanMacIdentityDisposition::RejectedMalformed)
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.identity_key_allowed()),
        Some(false)
    );
    assert_eq!(
        assessment
            .as_ref()
            .map(|assessment| assessment.stable_identity_key_allowed()),
        Some(false)
    );
}

#[test]
fn all_zero_mac_is_rejected_as_malformed() {
    let assessment = assess_mac_address(Some("00:00:00:00:00:00"));

    assert_eq!(
        assessment.map(|assessment| assessment.disposition()),
        Some(LanMacIdentityDisposition::RejectedMalformed)
    );
    assert!(normalize_scan_mac_address("00:00:00:00:00:00").is_none());
}

#[test]
fn compact_and_dotted_mac_forms_normalize_to_one_identity() {
    assert_eq!(
        normalize_scan_mac_address("54271e97c331"),
        Some("54-27-1e-97-c3-31".to_string())
    );
    assert_eq!(
        normalize_scan_mac_address("5427.1e97.c331"),
        Some("54-27-1e-97-c3-31".to_string())
    );
}
