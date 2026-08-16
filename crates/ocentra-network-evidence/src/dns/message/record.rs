use super::{data, name, DnsResourceRecord, NetworkReplayError};

pub(super) fn parse_resource_record(
    payload: &[u8],
    offset: usize,
) -> Result<(DnsResourceRecord, usize), NetworkReplayError> {
    let (record_name, metadata_offset) = name::parse_dns_name(payload, offset, true)?;
    if payload.len() < metadata_offset + 10 {
        return Err(NetworkReplayError::DnsResourceRecordTruncated);
    }

    let raw_record_type =
        u16::from_be_bytes([payload[metadata_offset], payload[metadata_offset + 1]]);
    let record_class =
        u16::from_be_bytes([payload[metadata_offset + 2], payload[metadata_offset + 3]]);
    let ttl_seconds = u32::from_be_bytes([
        payload[metadata_offset + 4],
        payload[metadata_offset + 5],
        payload[metadata_offset + 6],
        payload[metadata_offset + 7],
    ]);
    let data_len = usize::from(u16::from_be_bytes([
        payload[metadata_offset + 8],
        payload[metadata_offset + 9],
    ]));
    let data_offset = metadata_offset + 10;
    let next_offset = data_offset + data_len;
    let data = payload
        .get(data_offset..next_offset)
        .ok_or(NetworkReplayError::DnsResourceRecordTruncated)?;

    Ok((
        DnsResourceRecord {
            record_name,
            record_type: query_type(raw_record_type),
            record_class,
            ttl_seconds,
            data: data::dns_record_data(raw_record_type, data),
        },
        next_offset,
    ))
}

fn query_type(value: u16) -> super::DnsQueryType {
    match value {
        1 => super::DnsQueryType::A,
        28 => super::DnsQueryType::Aaaa,
        other => super::DnsQueryType::Unknown(other),
    }
}
