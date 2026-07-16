use super::super::{
    MdnsDnsSdRecord, MdnsRecordData, MDNS_TYPE_A, MDNS_TYPE_AAAA, MDNS_TYPE_PTR, MDNS_TYPE_SRV,
    MDNS_TYPE_TXT,
};
use super::parse_dns_name;
#[path = "packet_records_address.rs"]
mod packet_records_address;
#[path = "packet_records_text.rs"]
mod packet_records_text;

use packet_records_address::{
    ipv4_record_data, ipv6_record_data, parse_srv_record, ptr_record_data,
};
use packet_records_text::parse_txt_records;

pub(super) fn parse_resource_record(
    payload: &[u8],
    offset: usize,
) -> Option<(MdnsDnsSdRecord, usize)> {
    let (name, metadata_offset) = parse_dns_name(payload, offset)?;
    let (record_type, data_offset, data) = resource_record_metadata(payload, metadata_offset)?;
    let record_data = resource_record_data(payload, record_type, data_offset, data);

    Some((
        MdnsDnsSdRecord {
            name,
            data: record_data,
        },
        data_offset + data.len(),
    ))
}

pub(super) fn resource_record_data(
    payload: &[u8],
    record_type: u16,
    data_offset: usize,
    data: &[u8],
) -> MdnsRecordData {
    match record_type {
        MDNS_TYPE_PTR => ptr_record_data(payload, data_offset),
        MDNS_TYPE_SRV => parse_srv_record(payload, data_offset),
        MDNS_TYPE_TXT => MdnsRecordData::Txt(parse_txt_records(data)),
        MDNS_TYPE_A => ipv4_record_data(data),
        MDNS_TYPE_AAAA => ipv6_record_data(data),
        _ => MdnsRecordData::Unknown,
    }
}

fn resource_record_metadata(payload: &[u8], metadata_offset: usize) -> Option<(u16, usize, &[u8])> {
    let end_of_metadata = metadata_offset.checked_add(10)?;
    if end_of_metadata > payload.len() {
        return None;
    }

    let record_type = u16::from_be_bytes([payload[metadata_offset], payload[metadata_offset + 1]]);
    let data_len = usize::from(u16::from_be_bytes([
        payload[metadata_offset + 8],
        payload[metadata_offset + 9],
    ]));
    let data_offset = end_of_metadata;
    let data_end = data_offset.checked_add(data_len)?;
    let data = payload.get(data_offset..data_end)?;
    Some((record_type, data_offset, data))
}
