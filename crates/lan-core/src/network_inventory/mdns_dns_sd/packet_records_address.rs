use super::super::super::super::neighbor_support::normalize_neighbor_hostname;
use super::super::super::{parse_dns_name, MdnsRecordData};

pub(super) fn ipv4_record_data(data: &[u8]) -> MdnsRecordData {
    if data.len() == 4 {
        MdnsRecordData::A(format!("{}.{}.{}.{}", data[0], data[1], data[2], data[3]))
    } else {
        MdnsRecordData::Unknown
    }
}

pub(super) fn ipv6_record_data(data: &[u8]) -> MdnsRecordData {
    if data.len() != 16 {
        return MdnsRecordData::Unknown;
    }
    let address = std::net::Ipv6Addr::from([
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9],
        data[10], data[11], data[12], data[13], data[14], data[15],
    ]);
    MdnsRecordData::Aaaa(address.to_string())
}

pub(super) fn parse_srv_record(payload: &[u8], offset: usize) -> MdnsRecordData {
    let Some(data) = payload.get(offset..) else {
        return MdnsRecordData::Unknown;
    };
    if data.len() < 6 {
        return MdnsRecordData::Unknown;
    }
    let port = u16::from_be_bytes([data[4], data[5]]);
    let target_hostname = parse_dns_name(payload, offset + 6)
        .and_then(|(hostname, _)| normalize_neighbor_hostname(&hostname))
        .and_then(|hostname| (!hostname.is_empty()).then_some(hostname));
    MdnsRecordData::Srv {
        target_hostname,
        port: Some(port),
    }
}

pub(super) fn ptr_record_data(payload: &[u8], data_offset: usize) -> MdnsRecordData {
    parse_dns_name(payload, data_offset)
        .map(|(target, _)| MdnsRecordData::Ptr(target))
        .unwrap_or(MdnsRecordData::Unknown)
}
