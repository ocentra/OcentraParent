#[path = "packet_parse_records.rs"]
mod packet_parse_records;

use super::{parse_dns_name, MdnsDnsSdPacket, MDNS_HEADER_LEN};

pub(super) fn parse_mdns_packet(payload: &[u8]) -> Option<MdnsDnsSdPacket> {
    if payload.len() < MDNS_HEADER_LEN {
        return None;
    }
    let question_count = usize::from(u16::from_be_bytes([payload[4], payload[5]]));
    let answer_count = usize::from(u16::from_be_bytes([payload[6], payload[7]]));
    let authority_count = usize::from(u16::from_be_bytes([payload[8], payload[9]]));
    let additional_count = usize::from(u16::from_be_bytes([payload[10], payload[11]]));

    let offset = skip_mdns_questions(payload, question_count);
    let records = packet_parse_records::collect_mdns_records(
        payload,
        offset,
        [answer_count, authority_count, additional_count],
    );
    Some(MdnsDnsSdPacket { records })
}

pub(super) fn skip_mdns_questions(payload: &[u8], question_count: usize) -> usize {
    let mut offset = MDNS_HEADER_LEN;
    for _ in 0..question_count {
        let next_offset = match parse_dns_name(payload, offset) {
            Some((_, next_offset)) => next_offset,
            None => offset.saturating_add(1),
        };
        offset = match next_offset.checked_add(4) {
            Some(value) if value <= payload.len() => value,
            _ => break,
        };
    }
    offset
}
