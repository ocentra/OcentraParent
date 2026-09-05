use super::*;

#[test]
fn ssdp_udn_merges_same_device_even_when_neighbor_device_ids_differ() {
    let mut alpha = discovery_device(
        "lan-device-ssdp-alpha",
        None::<&str>,
        "Media Renderer",
        Some("renderer.local"),
        Some("192.168.1.100"),
        None::<&str>,
        vec![LanDiscoveryEvidenceSource::SsdpUpnpQuery],
    );
    alpha.service_identity_probe_evidence = vec![
        service_hint(
            LanServiceIdentityProbeEvidenceKind::SsdpDeviceType,
            "urn:schemas-upnp-org:device:MediaRenderer:1",
        ),
        service_hint(
            LanServiceIdentityProbeEvidenceKind::SsdpUdn,
            "uuid:media-renderer-1",
        ),
    ];

    let mut bravo = discovery_device(
        "lan-device-ssdp-bravo",
        None::<&str>,
        "Media Renderer",
        Some("renderer.local"),
        Some("192.168.1.101"),
        None::<&str>,
        vec![LanDiscoveryEvidenceSource::SsdpUpnpQuery],
    );
    bravo.service_identity_probe_evidence = alpha.service_identity_probe_evidence.clone();

    let model = build_lan_add_device_read_model(lan_input(vec![alpha, bravo]));

    assert_eq!(model.canonical_household_devices.len(), 1);
    assert_model_has_dedupe_note(&model, ["dedupe-decision=automatic", "shared-ssdp-udn"]);
    assert!(model.canonical_household_devices[0]
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint
                && record.confidence == LanDiscoveryEvidenceConfidence::Strong
                && record.value == "ssdp-udn:uuid:media-renderer-1"
        }));
}
