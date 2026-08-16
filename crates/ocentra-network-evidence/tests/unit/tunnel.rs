use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::tunnel::*;

#[derive(Clone, Copy)]
struct SourceRef(&'static str);

#[test]
fn tunnel_classifier_flags_vpn_adapter_indicator_without_hidden_destination_claim() {
    let classification = classify_vpn_proxy_tunnel_activity(NetworkTunnelClassifierInput {
        indicators: vec![NetworkTunnelIndicatorEvidence {
            indicator: NetworkTunnelIndicator::VpnAdapter,
            confidence_percent: 91,
            source_ref: "adapter-route-1".to_owned(),
        }],
    })
    .expect_value("vpn adapter indicator should classify");

    assert_tunnel_classification(
        &classification,
        NetworkTunnelKind::Vpn,
        NetworkTunnelBasis::VpnAdapterIndicator,
        91,
        SourceRef("adapter-route-1"),
    );
}

#[test]
fn tunnel_classifier_flags_proxy_port_indicator() {
    let classification = classify_vpn_proxy_tunnel_activity(NetworkTunnelClassifierInput {
        indicators: vec![NetworkTunnelIndicatorEvidence {
            indicator: NetworkTunnelIndicator::SocksProxyPort,
            confidence_percent: 78,
            source_ref: "flow-port-1080".to_owned(),
        }],
    })
    .expect_value("proxy port indicator should classify");

    assert_tunnel_classification(
        &classification,
        NetworkTunnelKind::Proxy,
        NetworkTunnelBasis::ProxyPortIndicator,
        78,
        SourceRef("flow-port-1080"),
    );
}

#[test]
fn tunnel_classifier_prioritizes_tor_over_generic_proxy_indicator() {
    let classification = classify_vpn_proxy_tunnel_activity(NetworkTunnelClassifierInput {
        indicators: vec![
            NetworkTunnelIndicatorEvidence {
                indicator: NetworkTunnelIndicator::SocksProxyPort,
                confidence_percent: 90,
                source_ref: "flow-port-1080".to_owned(),
            },
            NetworkTunnelIndicatorEvidence {
                indicator: NetworkTunnelIndicator::TorBootstrapDomain,
                confidence_percent: 84,
                source_ref: "tor-bootstrap-domain".to_owned(),
            },
        ],
    })
    .expect_value("tor indicator should classify");

    assert_eq!(classification.tunnel_kind, NetworkTunnelKind::Tor);
    assert_eq!(classification.basis, NetworkTunnelBasis::TorIndicator);
    assert_eq!(
        classification.evidence_refs,
        vec!["tor-bootstrap-domain".to_owned()]
    );
    assert!(!classification.hidden_destination_claimed);
}

#[test]
fn tunnel_classifier_does_not_claim_hidden_destination_from_encrypted_dns_only() {
    let classification = classify_vpn_proxy_tunnel_activity(NetworkTunnelClassifierInput {
        indicators: vec![NetworkTunnelIndicatorEvidence {
            indicator: NetworkTunnelIndicator::EncryptedDnsOnly,
            confidence_percent: 70,
            source_ref: "doh-candidate-1".to_owned(),
        }],
    })
    .expect_value("encrypted dns only should remain a negative tunnel proof");

    assert_eq!(classification.tunnel_kind, NetworkTunnelKind::Unknown);
    assert_eq!(
        classification.basis,
        NetworkTunnelBasis::EncryptedDnsOnlyNoTunnel
    );
    assert_eq!(classification.confidence_percent, 0);
    assert_eq!(
        classification.evidence_refs,
        vec!["doh-candidate-1".to_owned()]
    );
    assert!(!classification.hidden_destination_claimed);
    assert!(!classification.exact_url_available);
    assert!(!classification.decrypted_payload_available);
}

#[test]
fn tunnel_classifier_rejects_empty_indicator_source_ref() {
    let result = classify_vpn_proxy_tunnel_activity(NetworkTunnelClassifierInput {
        indicators: vec![NetworkTunnelIndicatorEvidence {
            indicator: NetworkTunnelIndicator::WireGuardUdpPort,
            confidence_percent: 82,
            source_ref: " ".to_owned(),
        }],
    });

    assert_eq!(
        result,
        Err(NetworkTunnelClassifierError::EmptyIndicatorSourceRef)
    );
}

fn assert_tunnel_classification(
    classification: &NetworkTunnelClassification,
    tunnel_kind: NetworkTunnelKind,
    basis: NetworkTunnelBasis,
    confidence_percent: u8,
    source_ref: SourceRef,
) {
    assert_eq!(classification.tunnel_kind, tunnel_kind);
    assert_eq!(classification.basis, basis);
    assert_eq!(classification.confidence_percent, confidence_percent);
    assert_eq!(classification.evidence_refs, vec![source_ref.0.to_owned()]);
    assert!(!classification.hidden_destination_claimed);
    assert!(!classification.exact_url_available);
    assert!(!classification.decrypted_payload_available);
}
