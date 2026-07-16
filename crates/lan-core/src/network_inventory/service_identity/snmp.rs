use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::time::Duration;

use super::{
    AllowedSnmpResponseObservation, LanServiceIdentityProbeObservation,
    SERVICE_IDENTITY_PROBE_MAX_RESPONSE_BYTES, SERVICE_IDENTITY_PROBE_READ_TIMEOUT_MS,
    SNMP_REQUEST_ID,
};

pub mod parse;

mod ber;

pub fn probe_snmp_identity_query(
    ip_address: &str,
    allowed_snmp_response_observer: super::AllowedSnmpResponseObserver<'_>,
) -> Option<LanServiceIdentityProbeObservation> {
    let ip_address = ip_address.parse::<Ipv4Addr>().ok()?;
    let endpoint = SocketAddr::new(ip_address.into(), 161);
    probe_snmp_identity_query_at_endpoint(endpoint, allowed_snmp_response_observer)
}

pub fn probe_snmp_identity_query_at_endpoint(
    endpoint: SocketAddr,
    allowed_snmp_response_observer: super::AllowedSnmpResponseObserver<'_>,
) -> Option<LanServiceIdentityProbeObservation> {
    let socket = UdpSocket::bind(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 0)).ok()?;
    let read_timeout = Some(Duration::from_millis(
        SERVICE_IDENTITY_PROBE_READ_TIMEOUT_MS,
    ));
    let _ = socket.set_read_timeout(read_timeout);
    let _ = socket.set_write_timeout(read_timeout);
    let request = encode_snmp_identity_request(SNMP_REQUEST_ID);
    socket.send_to(&request, endpoint).ok()?;
    let mut response = vec![0_u8; SERVICE_IDENTITY_PROBE_MAX_RESPONSE_BYTES.min(2048)];
    let (read, _) = socket.recv_from(&mut response).ok()?;
    let response = &response[..read];
    let observation = parse_snmp_probe_observation(response, SNMP_REQUEST_ID)?;
    if let Some(observer) = allowed_snmp_response_observer {
        observer(response);
    }
    Some(observation)
}

pub fn parse_allowed_snmp_response(response: &[u8]) -> Option<AllowedSnmpResponseObservation> {
    let observation = parse_snmp_probe_observation(response, SNMP_REQUEST_ID)?;
    let parsed = AllowedSnmpResponseObservation {
        sys_descr: observation.snmp_sys_descr,
        sys_name: observation.snmp_sys_name,
    };
    (parsed.sys_descr.is_some() || parsed.sys_name.is_some()).then_some(parsed)
}

pub fn parse_snmp_probe_observation(
    response: &[u8],
    expected_request_id: i64,
) -> Option<LanServiceIdentityProbeObservation> {
    parse::parse_snmp_probe_observation(response, expected_request_id)
}

pub fn encode_snmp_identity_request(request_id: i64) -> Vec<u8> {
    ber::encode_snmp_identity_request(request_id)
}

pub fn encode_snmp_varbind_request(oid: &[u32]) -> Vec<u8> {
    ber::encode_snmp_varbind_request(oid)
}

pub fn encode_ber_tlv(tag: u8, value: &[u8]) -> Vec<u8> {
    ber::encode_ber_tlv(tag, value)
}

pub fn encode_ber_length(length: usize) -> Vec<u8> {
    ber::encode_ber_length(length)
}

pub fn encode_ber_integer(value: i64) -> Vec<u8> {
    ber::encode_ber_integer(value)
}

pub fn encode_ber_oid(oid: &[u32]) -> Vec<u8> {
    ber::encode_ber_oid(oid)
}

pub fn parse_ber_tlv(payload: &[u8], offset: usize) -> Option<(u8, &[u8], usize)> {
    ber::parse_ber_tlv(payload, offset)
}

pub fn parse_ber_integer(payload: &[u8]) -> Option<i64> {
    ber::parse_ber_integer(payload)
}

pub fn parse_ber_oid(payload: &[u8]) -> Option<Vec<u32>> {
    ber::parse_ber_oid(payload)
}
