use crate::support::{OptionTestExt as _, ResultTestExt as _};
use std::fs;

use ocentra_lan_core::network_inventory::name_evidence::*;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceSource,
};

#[macro_use]
#[path = "network_inventory_name_evidence_fixture_support.rs"]
mod fixture_support;

#[test]
fn duplicate_name_fixture_keeps_name_only_evidence_weak_and_time_scoped() {
    let evidences = fs::read_to_string(format!(
        "{}/tests/fixtures/lan-plan/{}",
        env!("CARGO_MANIFEST_DIR"),
        "name_evidence_duplicate_names.txt"
    ))
    .value_or_unreachable()
    .lines()
    .filter_map(|line| {
        let mut columns = line.split('|');
        let source = columns.next()?;
        let value = columns.next()?;
        let observed_at = columns.next()?;
        let network_interface = columns.next();
        weak_name_evidence_from_source!(source, value, observed_at, network_interface)
    })
    .collect::<Vec<_>>();

    assert_eq!(evidences.len(), 6);

    for pair in evidences.chunks_exact(2) {
        let first = &pair[0];
        let second = &pair[1];

        assert_eq!(first.source, second.source);
        assert_eq!(first.confidence, LanDiscoveryEvidenceConfidence::Weak);
        assert_eq!(second.confidence, LanDiscoveryEvidenceConfidence::Weak);
        assert_eq!(first.value, second.value);
        assert_eq!(first.normalized_value, second.normalized_value);
        assert_eq!(first.network_interface, second.network_interface);
        assert_ne!(first.first_seen_at, second.first_seen_at);
        assert_eq!(first.last_seen_at, first.first_seen_at);
        assert_eq!(second.last_seen_at, second.first_seen_at);
    }
}

#[test]
fn long_name_fixture_is_rejected_for_all_supported_weak_name_sources() {
    let long_name = fs::read_to_string(format!(
        "{}/tests/fixtures/lan-plan/{}",
        env!("CARGO_MANIFEST_DIR"),
        "name_evidence_long_names.txt"
    ))
    .value_or_unreachable();
    let long_name = long_name.trim_end_matches(['\r', '\n']);

    for source in ["reverse-dns", "netbios", "llmnr"] {
        assert!(
            weak_name_evidence_from_source!(
                source,
                long_name,
                "2026-06-26T00:00:00Z",
                Some("Wi-Fi")
            )
            .is_none(),
            "{source} should reject oversized weak name evidence"
        );
    }
}

#[test]
fn name_evidence_tracks_source_confidence_timing_and_interface() {
    let evidence = llmnr_name_evidence(
        " Kid-Laptop.local. ",
        "2026-06-26T00:00:00Z",
        Some(" Wi-Fi "),
    )
    .value_or_unreachable();

    assert_eq!(evidence.source, LanDiscoveryEvidenceSource::Llmnr);
    assert_eq!(evidence.confidence, LanDiscoveryEvidenceConfidence::Weak);
    assert_eq!(evidence.value, "Kid-Laptop.local");
    assert_eq!(evidence.normalized_value, "kid-laptop.local");
    assert_eq!(evidence.first_seen_at, "2026-06-26T00:00:00Z");
    assert_eq!(evidence.last_seen_at, "2026-06-26T00:00:00Z");
    assert_eq!(evidence.network_interface.as_deref(), Some("Wi-Fi"));
    assert_eq!(
        evidence.source_label(),
        constants::lan_pairing::LAN_SCAN_SOURCE_LLMNR
    );
    assert_eq!(evidence.confidence_label(), "weak");
}

#[test]
fn all_supported_name_sources_normalize_to_weak_evidence_with_trimmed_interface() {
    let cases = [
        (
            dns_cache_name_evidence(
                " Printer-1.example.local. ",
                "2026-06-26T00:00:00Z",
                Some(" Ethernet "),
            ),
            LanDiscoveryEvidenceSource::DnsCache,
            Some("Ethernet"),
        ),
        (
            reverse_dns_name_evidence(
                " Printer-1.example.local. ",
                "2026-06-26T00:00:00Z",
                Some(" Ethernet "),
            ),
            LanDiscoveryEvidenceSource::DnsCache,
            Some("Ethernet"),
        ),
        (
            netbios_name_evidence(" WORKSTATION-1 ", "2026-06-26T00:00:00Z", Some(" Wi-Fi ")),
            LanDiscoveryEvidenceSource::Netbios,
            Some("Wi-Fi"),
        ),
        (
            llmnr_name_evidence(" WORKSTATION-2 ", "2026-06-26T00:00:00Z", Some(" Wi-Fi ")),
            LanDiscoveryEvidenceSource::Llmnr,
            Some("Wi-Fi"),
        ),
    ];

    for (evidence, expected_source, expected_interface) in cases {
        let evidence = evidence.value_or_unreachable();
        assert_eq!(evidence.source, expected_source);
        assert_eq!(evidence.confidence, LanDiscoveryEvidenceConfidence::Weak);
        assert_eq!(evidence.first_seen_at, "2026-06-26T00:00:00Z");
        assert_eq!(evidence.last_seen_at, "2026-06-26T00:00:00Z");
        assert_eq!(evidence.network_interface.as_deref(), expected_interface);
        assert_eq!(
            evidence.normalized_value,
            evidence.value.to_ascii_lowercase()
        );
    }
}

#[test]
fn duplicate_name_observations_stay_weak_and_time_scoped() {
    let observed_name = " Kid-Laptop.local. ";
    let cases = [
        (
            reverse_dns_name_evidence(observed_name, "2026-06-26T00:00:00Z", Some(" Wi-Fi ")),
            reverse_dns_name_evidence(observed_name, "2026-06-26T00:05:00Z", Some(" Wi-Fi ")),
            LanDiscoveryEvidenceSource::DnsCache,
        ),
        (
            netbios_name_evidence(observed_name, "2026-06-26T00:00:00Z", Some(" Wi-Fi ")),
            netbios_name_evidence(observed_name, "2026-06-26T00:05:00Z", Some(" Wi-Fi ")),
            LanDiscoveryEvidenceSource::Netbios,
        ),
        (
            llmnr_name_evidence(observed_name, "2026-06-26T00:00:00Z", Some(" Wi-Fi ")),
            llmnr_name_evidence(observed_name, "2026-06-26T00:05:00Z", Some(" Wi-Fi ")),
            LanDiscoveryEvidenceSource::Llmnr,
        ),
    ];

    for (first, second, expected_source) in cases {
        let first = first.value_or_unreachable();
        let second = second.value_or_unreachable();

        assert_eq!(first.source, expected_source);
        assert_eq!(second.source, expected_source);
        assert_eq!(first.confidence, LanDiscoveryEvidenceConfidence::Weak);
        assert_eq!(second.confidence, LanDiscoveryEvidenceConfidence::Weak);
        assert_eq!(first.value, "Kid-Laptop.local");
        assert_eq!(second.value, "Kid-Laptop.local");
        assert_eq!(first.normalized_value, "kid-laptop.local");
        assert_eq!(second.normalized_value, "kid-laptop.local");
        assert_eq!(first.network_interface.as_deref(), Some("Wi-Fi"));
        assert_eq!(second.network_interface.as_deref(), Some("Wi-Fi"));
        assert_eq!(first.first_seen_at, "2026-06-26T00:00:00Z");
        assert_eq!(first.last_seen_at, "2026-06-26T00:00:00Z");
        assert_eq!(second.first_seen_at, "2026-06-26T00:05:00Z");
        assert_eq!(second.last_seen_at, "2026-06-26T00:05:00Z");
    }
}

#[test]
fn name_evidence_rejects_snsafe_oversized_and_empty_names() {
    assert!(netbios_name_evidence("bad host", "2026-06-26T00:00:00Z", None).is_none());
    assert!(dns_cache_name_evidence("bad<script>", "2026-06-26T00:00:00Z", None).is_none());
    assert!(llmnr_name_evidence("kid-laptop.local..", "2026-06-26T00:00:00Z", None).is_none());
    assert!(reverse_dns_name_evidence(
        &"a".repeat(MAX_NAME_EVIDENCE_BYTES + 1),
        "2026-06-26T00:00:00Z",
        None
    )
    .is_none());
    assert!(netbios_name_evidence(
        &"a".repeat(MAX_NAME_EVIDENCE_BYTES + 1),
        "2026-06-26T00:00:00Z",
        None
    )
    .is_none());
    assert!(llmnr_name_evidence(
        &"a".repeat(MAX_NAME_EVIDENCE_BYTES + 1),
        "2026-06-26T00:00:00Z",
        None
    )
    .is_none());
    assert!(reverse_dns_name_evidence("printer.local", "", None).is_none());
}

#[test]
fn name_evidence_enforces_hostname_label_length_bosndaries() {
    let valid_label = "a".repeat(63);
    let invalid_label = "a".repeat(64);

    let netbios = netbios_name_evidence(&valid_label, "2026-06-26T00:00:00Z", None);
    assert_eq!(
        netbios.as_ref().map(|evidence| evidence.source.clone()),
        Some(LanDiscoveryEvidenceSource::Netbios)
    );
    assert_eq!(
        netbios.as_ref().map(|evidence| evidence.confidence.clone()),
        Some(LanDiscoveryEvidenceConfidence::Weak)
    );
    assert_eq!(
        netbios.as_ref().map(|evidence| evidence.value.as_str()),
        Some(valid_label.as_str())
    );
    assert_eq!(
        netbios
            .as_ref()
            .map(|evidence| evidence.normalized_value.as_str()),
        Some(valid_label.as_str())
    );

    let expected_llmnr_value = format!("{valid_label}.local");
    let llmnr = llmnr_name_evidence(&expected_llmnr_value, "2026-06-26T00:00:00Z", Some("Wi-Fi"));
    assert_eq!(
        llmnr.as_ref().map(|evidence| evidence.source.clone()),
        Some(LanDiscoveryEvidenceSource::Llmnr)
    );
    assert_eq!(
        llmnr.as_ref().map(|evidence| evidence.confidence.clone()),
        Some(LanDiscoveryEvidenceConfidence::Weak)
    );
    assert_eq!(
        llmnr.as_ref().map(|evidence| evidence.value.as_str()),
        Some(expected_llmnr_value.as_str())
    );
    assert_eq!(
        llmnr
            .as_ref()
            .map(|evidence| evidence.normalized_value.as_str()),
        Some(expected_llmnr_value.as_str())
    );
    assert!(netbios_name_evidence(&invalid_label, "2026-06-26T00:00:00Z", None).is_none());
    assert!(reverse_dns_name_evidence(
        &format!("{invalid_label}.local"),
        "2026-06-26T00:00:00Z",
        None,
    )
    .is_none());
}
