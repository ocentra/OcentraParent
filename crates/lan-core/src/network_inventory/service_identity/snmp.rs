use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::time::Duration;

use super::{
    AllowedSnmpResponseObservation, LanServiceIdentityProbeObservation, BER_TAG_INTEGER,
    BER_TAG_NULL, BER_TAG_OBJECT_IDENTIFIER, BER_TAG_OCTET_STRING, BER_TAG_SEQUENCE,
    SERVICE_IDENTITY_PROBE_MAX_RESPONSE_BYTES, SERVICE_IDENTITY_PROBE_READ_TIMEOUT_MS,
    SNMP_GET_REQUEST_TAG, SNMP_PUBLIC_COMMUNITY, SNMP_REQUEST_ID, SNMP_SYS_DESCR_OID,
    SNMP_SYS_NAME_OID, SNMP_VERSION_2C,
};

pub mod parse;

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
    let version = encode_ber_tlv(BER_TAG_INTEGER, &encode_ber_integer(SNMP_VERSION_2C));
    let community = encode_ber_tlv(BER_TAG_OCTET_STRING, SNMP_PUBLIC_COMMUNITY.as_bytes());
    let sys_descr_varbind = encode_snmp_varbind_request(SNMP_SYS_DESCR_OID);
    let sys_name_varbind = encode_snmp_varbind_request(SNMP_SYS_NAME_OID);
    let varbind_list = encode_ber_tlv(
        BER_TAG_SEQUENCE,
        &[sys_descr_varbind, sys_name_varbind].concat(),
    );
    let pdu = encode_ber_tlv(
        SNMP_GET_REQUEST_TAG,
        &[
            encode_ber_tlv(BER_TAG_INTEGER, &encode_ber_integer(request_id)),
            encode_ber_tlv(BER_TAG_INTEGER, &encode_ber_integer(0)),
            encode_ber_tlv(BER_TAG_INTEGER, &encode_ber_integer(0)),
            varbind_list,
        ]
        .concat(),
    );
    encode_ber_tlv(BER_TAG_SEQUENCE, &[version, community, pdu].concat())
}

pub fn encode_snmp_varbind_request(oid: &[u32]) -> Vec<u8> {
    encode_ber_tlv(
        BER_TAG_SEQUENCE,
        &[
            encode_ber_tlv(BER_TAG_OBJECT_IDENTIFIER, &encode_ber_oid(oid)),
            encode_ber_tlv(BER_TAG_NULL, &[]),
        ]
        .concat(),
    )
}

pub fn encode_ber_tlv(tag: u8, value: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(1 + value.len() + 5);
    encoded.push(tag);
    encoded.extend_from_slice(&encode_ber_length(value.len()));
    encoded.extend_from_slice(value);
    encoded
}

pub fn encode_ber_length(length: usize) -> Vec<u8> {
    if length < 0x80 {
        return vec![u8::try_from(length).ok().unwrap_or(0)];
    }
    let mut bytes = Vec::new();
    let mut remaining = length;
    while remaining > 0 {
        bytes.push((remaining & 0xff) as u8);
        remaining >>= 8;
    }
    bytes.reverse();
    let mut encoded = Vec::with_capacity(bytes.len() + 1);
    encoded.push(0x80 | u8::try_from(bytes.len()).ok().unwrap_or(0));
    encoded.extend_from_slice(&bytes);
    encoded
}

pub fn encode_ber_integer(value: i64) -> Vec<u8> {
    let mut bytes = value.to_be_bytes().to_vec();
    while bytes.len() > 1
        && ((bytes[0] == 0x00 && (bytes[1] & 0x80) == 0)
            || (bytes[0] == 0xff && (bytes[1] & 0x80) == 0x80))
    {
        bytes.remove(0);
    }
    bytes
}

pub fn encode_ber_oid(oid: &[u32]) -> Vec<u8> {
    let Some((&first, rest)) = oid.split_first() else {
        return Vec::new();
    };
    let Some((&second, tail)) = rest.split_first() else {
        return Vec::new();
    };
    let mut encoded = vec![
        u8::try_from(first.saturating_mul(40).saturating_add(second))
            .ok()
            .unwrap_or(0),
    ];
    for component in tail {
        let mut stack = vec![u8::try_from(component & 0x7f).ok().unwrap_or(0)];
        let mut value = *component >> 7;
        while value > 0 {
            stack.push(u8::try_from(value & 0x7f).ok().unwrap_or(0) | 0x80);
            value >>= 7;
        }
        stack.reverse();
        encoded.extend_from_slice(&stack);
    }
    encoded
}

pub fn parse_ber_tlv(payload: &[u8], offset: usize) -> Option<(u8, &[u8], usize)> {
    let tag = *payload.get(offset)?;
    let length_first = *payload.get(offset + 1)?;
    let (length, value_offset) = if (length_first & 0x80) == 0 {
        (usize::from(length_first), offset + 2)
    } else {
        let length_len = usize::from(length_first & 0x7f);
        if length_len == 0 || length_len > std::mem::size_of::<usize>() {
            return None;
        }
        let mut decoded_length = 0_usize;
        for byte in payload.get((offset + 2)..(offset + 2 + length_len))? {
            decoded_length = (decoded_length << 8) | usize::from(*byte);
        }
        (decoded_length, offset + 2 + length_len)
    };
    let value_end = value_offset.checked_add(length)?;
    Some((tag, payload.get(value_offset..value_end)?, value_end))
}

pub fn parse_ber_integer(payload: &[u8]) -> Option<i64> {
    if payload.is_empty() || payload.len() > 8 {
        return None;
    }
    let negative = (payload[0] & 0x80) != 0;
    let mut bytes = [if negative { 0xff } else { 0x00 }; 8];
    let start = 8_usize.checked_sub(payload.len())?;
    bytes[start..].copy_from_slice(payload);
    Some(i64::from_be_bytes(bytes))
}

pub fn parse_ber_oid(payload: &[u8]) -> Option<Vec<u32>> {
    let first = *payload.first()?;
    let mut oid = vec![u32::from(first / 40), u32::from(first % 40)];
    let mut index = 1_usize;
    while index < payload.len() {
        let mut value = 0_u32;
        loop {
            let byte = *payload.get(index)?;
            index += 1;
            value = (value << 7) | u32::from(byte & 0x7f);
            if (byte & 0x80) == 0 {
                break;
            }
        }
        oid.push(value);
    }
    Some(oid)
}
