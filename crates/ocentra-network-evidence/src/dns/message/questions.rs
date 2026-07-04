use super::{constants, name, DnsQueryType, DnsQuestion, NetworkReplayError};

pub(super) fn parse_questions(
    payload: &[u8],
    question_count: usize,
) -> Result<(Vec<DnsQuestion>, usize), NetworkReplayError> {
    let mut offset = constants::DNS_HEADER_LEN;
    let mut questions = Vec::new();
    for _ in 0..question_count {
        let (query_name, next_offset) = name::parse_dns_name(payload, offset, false)?;
        if payload.len() < next_offset + 4 {
            return Err(NetworkReplayError::DnsQuestionTruncated);
        }

        let raw_query_type = u16::from_be_bytes([payload[next_offset], payload[next_offset + 1]]);
        let query_class = u16::from_be_bytes([payload[next_offset + 2], payload[next_offset + 3]]);
        questions.push(DnsQuestion {
            query_name,
            query_type: query_type(raw_query_type),
            query_class,
        });
        offset = next_offset + 4;
    }

    Ok((questions, offset))
}

fn query_type(value: u16) -> DnsQueryType {
    match value {
        constants::DNS_TYPE_A => DnsQueryType::A,
        constants::DNS_TYPE_AAAA => DnsQueryType::Aaaa,
        other => DnsQueryType::Unknown(other),
    }
}
