use super::*;
use ocentra_lan_core::network_inventory::service_identity::snmp::{
    encode_ber_integer, encode_ber_oid, encode_ber_tlv, parse_snmp_probe_observation,
};

#[test]
fn snmp_identity_query_adds_weak_metadata_evidence_for_discovered_host() {
    let response = snmp_identity_query_response();

    let observation =
        parse_snmp_probe_observation(&response, SNMP_REQUEST_ID).value_or_unreachable();

    let evidence = observation.into_evidence();
    assert!(evidence.iter().any(|item| {
        item.evidence_kind == LanServiceIdentityProbeEvidenceKind::SnmpSysDescr
            && item.value == "Linux camera controller"
    }));
    assert!(evidence.iter().any(|item| {
        item.evidence_kind == LanServiceIdentityProbeEvidenceKind::SnmpSysName
            && item.value == "cam-1"
    }));
    assert!(!evidence
        .iter()
        .any(|item| { item.evidence_kind == LanServiceIdentityProbeEvidenceKind::HttpStatus }));
}

#[test]
fn snmp_identity_query_executes_against_local_udp_endpoint() {
    let socket = UdpSocket::bind("127.0.0.1:0").value_or_unreachable();
    let endpoint = socket.local_addr().value_or_unreachable();
    let server = thread::spawn(move || {
        let mut request = [0_u8; 1024];
        let (read, source) = socket.recv_from(&mut request).value_or_unreachable();
        assert_eq!(
            request[..read].to_vec(),
            encode_snmp_identity_request(SNMP_REQUEST_ID)
        );
        socket
            .send_to(&snmp_identity_query_response(), source)
            .value_or_unreachable();
    });

    let observation = probe_snmp_identity_query_at_endpoint(endpoint, None).value_or_unreachable();

    server.join().value_or_unreachable();

    let evidence = observation.into_evidence();
    assert!(evidence.iter().any(|item| {
        item.evidence_kind == LanServiceIdentityProbeEvidenceKind::SnmpSysDescr
            && item.value == "Linux camera controller"
    }));
    assert!(evidence.iter().any(|item| {
        item.evidence_kind == LanServiceIdentityProbeEvidenceKind::SnmpSysName
            && item.value == "cam-1"
    }));
}

#[test]
fn snmp_identity_query_notifies_allowed_snmp_observer_with_received_payload() {
    let socket = UdpSocket::bind("127.0.0.1:0").value_or_unreachable();
    let endpoint = socket.local_addr().value_or_unreachable();
    let expected_response = snmp_identity_query_response();
    let server_response = expected_response.clone();
    let server = thread::spawn(move || {
        let mut request = [0_u8; 1024];
        let (_, source) = socket.recv_from(&mut request).value_or_unreachable();
        socket
            .send_to(&server_response, source)
            .value_or_unreachable();
    });
    let observed_payload = std::sync::Mutex::new(Vec::new());

    let observation = probe_snmp_identity_query_at_endpoint(
        endpoint,
        Some(&|payload| {
            if let Ok(mut payloads) = observed_payload.lock() {
                payloads.push(payload.to_vec());
            }
        }),
    )
    .value_or_unreachable();

    server.join().value_or_unreachable();

    assert!(observation.observed_allowed_snmp_response());
    let observed_payload = observed_payload.lock().value_or_unreachable();
    assert_eq!(observed_payload.as_slice(), &[expected_response]);
}

fn snmp_identity_query_response() -> Vec<u8> {
    let varbind_list = encode_ber_tlv(
        BER_TAG_SEQUENCE,
        &[
            encode_ber_tlv(
                BER_TAG_SEQUENCE,
                &[
                    encode_ber_tlv(
                        BER_TAG_OBJECT_IDENTIFIER,
                        &encode_ber_oid(SNMP_SYS_DESCR_OID),
                    ),
                    encode_ber_tlv(BER_TAG_OCTET_STRING, b"Linux camera controller"),
                ]
                .concat(),
            ),
            encode_ber_tlv(
                BER_TAG_SEQUENCE,
                &[
                    encode_ber_tlv(
                        BER_TAG_OBJECT_IDENTIFIER,
                        &encode_ber_oid(SNMP_SYS_NAME_OID),
                    ),
                    encode_ber_tlv(BER_TAG_OCTET_STRING, b"cam-1"),
                ]
                .concat(),
            ),
        ]
        .concat(),
    );
    encode_ber_tlv(
        BER_TAG_SEQUENCE,
        &[
            encode_ber_tlv(BER_TAG_INTEGER, &encode_ber_integer(SNMP_VERSION_2C)),
            encode_ber_tlv(BER_TAG_OCTET_STRING, SNMP_PUBLIC_COMMUNITY.as_bytes()),
            encode_ber_tlv(
                SNMP_GET_RESPONSE_TAG,
                &[
                    encode_ber_tlv(BER_TAG_INTEGER, &encode_ber_integer(SNMP_REQUEST_ID)),
                    encode_ber_tlv(BER_TAG_INTEGER, &encode_ber_integer(0)),
                    encode_ber_tlv(BER_TAG_INTEGER, &encode_ber_integer(0)),
                    varbind_list,
                ]
                .concat(),
            ),
        ]
        .concat(),
    )
}
