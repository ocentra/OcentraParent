use super::super::{
    BER_TAG_INTEGER, BER_TAG_NULL, BER_TAG_OBJECT_IDENTIFIER, BER_TAG_OCTET_STRING,
    BER_TAG_SEQUENCE, SNMP_GET_REQUEST_TAG, SNMP_PUBLIC_COMMUNITY, SNMP_SYS_DESCR_OID,
    SNMP_SYS_NAME_OID, SNMP_VERSION_2C,
};

mod parse;

pub(super) fn encode_snmp_identity_request(request_id: i64) -> Vec<u8> {
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

pub(super) fn encode_snmp_varbind_request(oid: &[u32]) -> Vec<u8> {
    encode_ber_tlv(
        BER_TAG_SEQUENCE,
        &[
            encode_ber_tlv(BER_TAG_OBJECT_IDENTIFIER, &encode_ber_oid(oid)),
            encode_ber_tlv(BER_TAG_NULL, &[]),
        ]
        .concat(),
    )
}

pub(super) fn encode_ber_tlv(tag: u8, value: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(1 + value.len() + 5);
    encoded.push(tag);
    encoded.extend_from_slice(&encode_ber_length(value.len()));
    encoded.extend_from_slice(value);
    encoded
}

pub(super) fn encode_ber_length(length: usize) -> Vec<u8> {
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

pub(super) fn encode_ber_integer(value: i64) -> Vec<u8> {
    let mut bytes = value.to_be_bytes().to_vec();
    while bytes.len() > 1
        && ((bytes[0] == 0x00 && (bytes[1] & 0x80) == 0)
            || (bytes[0] == 0xff && (bytes[1] & 0x80) == 0x80))
    {
        bytes.remove(0);
    }
    bytes
}

pub(super) fn encode_ber_oid(oid: &[u32]) -> Vec<u8> {
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

pub(super) fn parse_ber_tlv(payload: &[u8], offset: usize) -> Option<(u8, &[u8], usize)> {
    parse::parse_ber_tlv(payload, offset)
}

pub(super) fn parse_ber_integer(payload: &[u8]) -> Option<i64> {
    parse::parse_ber_integer(payload)
}

pub(super) fn parse_ber_oid(payload: &[u8]) -> Option<Vec<u32>> {
    parse::parse_ber_oid(payload)
}
