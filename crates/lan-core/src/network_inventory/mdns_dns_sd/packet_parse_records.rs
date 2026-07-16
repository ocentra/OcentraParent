use super::super::super::MdnsDnsSdRecord;
use super::super::packet_records::parse_resource_record;

pub(super) fn collect_mdns_records(
    payload: &[u8],
    mut offset: usize,
    record_counts: [usize; 3],
) -> Vec<MdnsDnsSdRecord> {
    let mut records = Vec::new();
    for record_count in record_counts {
        offset = collect_mdns_records_for_count(payload, offset, record_count, &mut records);
    }
    records
}

fn collect_mdns_records_for_count(
    payload: &[u8],
    mut offset: usize,
    record_count: usize,
    records: &mut Vec<MdnsDnsSdRecord>,
) -> usize {
    for _ in 0..record_count {
        let Some((record, next_offset)) = parse_resource_record(payload, offset) else {
            let Some(next_offset) = advance_mdns_offset(offset, payload.len()) else {
                break;
            };
            offset = next_offset;
            continue;
        };
        if !is_valid_mdns_record_offset(offset, next_offset, payload.len()) {
            break;
        }
        records.push(record);
        offset = next_offset;
    }
    offset
}

fn advance_mdns_offset(offset: usize, payload_len: usize) -> Option<usize> {
    offset
        .checked_add(1)
        .filter(|next_offset| *next_offset < payload_len)
}

fn is_valid_mdns_record_offset(offset: usize, next_offset: usize, payload_len: usize) -> bool {
    next_offset > offset && next_offset <= payload_len
}
