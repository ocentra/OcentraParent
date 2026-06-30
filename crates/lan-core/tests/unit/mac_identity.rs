use ocentra_lan_core::mac_identity::{
    assess_mac_address, normalize_scan_mac_address, LanMacIdentityDisposition,
};
use ocentra_parent_agent_protocol::constants;

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
    assert!(assessment.stable_identity_key_allowed());
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
    assert!(assessment.stable_identity_key_allowed());
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
    assert!(!assessment.stable_identity_key_allowed());
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
    assert!(!assessment.stable_identity_key_allowed());
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
    assert!(!assessment.stable_identity_key_allowed());
}
