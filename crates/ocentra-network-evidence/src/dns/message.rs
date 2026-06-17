use super::{
    DnsMessage, DnsQueryType, DnsQuestion, DnsRecordData, DnsResourceRecord, NetworkReplayError,
};

const DNS_HEADER_LEN: usize = 12;
const DNS_RESPONSE_FLAG: u16 = 0x8000;
const DNS_POINTER_MASK: u8 = 0b1100_0000;
const DNS_POINTER_VALUE: u8 = 0b1100_0000;
const DNS_MAX_POINTER_JUMPS: usize = 8;
const DNS_TYPE_A: u16 = 1;
const DNS_TYPE_AAAA: u16 = 28;
const IPV4_RDATA_LEN: usize = 4;

pub fn parse_dns_message(payload: &[u8]) -> Result<DnsMessage, NetworkReplayError> {
    if payload.len() < DNS_HEADER_LEN {
        return Err(NetworkReplayError::DnsPacketTooShort);
    }

    let transaction_id = u16::from_be_bytes([payload[0], payload[1]]);
    let flags = u16::from_be_bytes([payload[2], payload[3]]);
    let question_count = u16::from_be_bytes([payload[4], payload[5]]);
    let answer_count = u16::from_be_bytes([payload[6], payload[7]]);
    if question_count == 0 {
        return Err(NetworkReplayError::DnsQuestionMissing);
    }

    let (questions, offset) = parse_questions(payload, usize::from(question_count))?;
    let answers = parse_answers(payload, offset, usize::from(answer_count))?;
    Ok(DnsMessage {
        transaction_id,
        is_response: flags & DNS_RESPONSE_FLAG != 0,
        questions,
        answers,
    })
}

fn parse_questions(
    payload: &[u8],
    question_count: usize,
) -> Result<(Vec<DnsQuestion>, usize), NetworkReplayError> {
    let mut offset = DNS_HEADER_LEN;
    let mut questions = Vec::new();
    for _ in 0..question_count {
        let (query_name, next_offset) = parse_dns_name(payload, offset, false)?;
        if payload.len() < next_offset + 4 {
            return Err(NetworkReplayError::DnsQuestionTruncated);
        }

        let raw_query_type = u16::from_be_bytes([payload[next_offset], payload[next_offset + 1]]);
        let query_class = u16::from_be_bytes([payload[next_offset + 2], payload[next_offset + 3]]);
        questions.push(DnsQuestion {
            query_name,
            query_type: dns_query_type(raw_query_type),
            query_class,
        });
        offset = next_offset + 4;
    }

    Ok((questions, offset))
}

fn parse_answers(
    payload: &[u8],
    mut offset: usize,
    answer_count: usize,
) -> Result<Vec<DnsResourceRecord>, NetworkReplayError> {
    let mut answers = Vec::new();
    for _ in 0..answer_count {
        let (answer, next_offset) = parse_resource_record(payload, offset)?;
        answers.push(answer);
        offset = next_offset;
    }

    Ok(answers)
}

fn parse_resource_record(
    payload: &[u8],
    offset: usize,
) -> Result<(DnsResourceRecord, usize), NetworkReplayError> {
    let (record_name, metadata_offset) = parse_dns_name(payload, offset, true)?;
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
            record_type: dns_query_type(raw_record_type),
            record_class,
            ttl_seconds,
            data: dns_record_data(raw_record_type, data),
        },
        next_offset,
    ))
}

fn parse_dns_name(
    payload: &[u8],
    offset: usize,
    allow_compression: bool,
) -> Result<(String, usize), NetworkReplayError> {
    let mut labels = Vec::new();
    let mut cursor = offset;
    let mut next_offset = offset;
    let mut jumped = false;
    let mut jumps = 0_usize;

    loop {
        let label_len = *payload
            .get(cursor)
            .ok_or(NetworkReplayError::DnsLabelOutOfBounds)?;
        if label_len == 0 {
            cursor += 1;
            if !jumped {
                next_offset = cursor;
            }
            break;
        }

        if label_len & DNS_POINTER_MASK == DNS_POINTER_VALUE {
            if !allow_compression {
                return Err(NetworkReplayError::DnsCompressedQuestionName);
            }
            let pointer_target = dns_pointer_target(payload, cursor, label_len)?;
            if !jumped {
                next_offset = cursor + 2;
            }
            cursor = pointer_target;
            jumped = true;
            jumps += 1;
            if jumps > DNS_MAX_POINTER_JUMPS {
                return Err(NetworkReplayError::DnsNamePointerLoop);
            }
            continue;
        }
        if label_len & DNS_POINTER_MASK != 0 {
            return Err(NetworkReplayError::DnsUnsupportedLabelMode);
        }

        cursor += 1;
        let label_len = usize::from(label_len);
        let label_end = cursor + label_len;
        let label = payload
            .get(cursor..label_end)
            .ok_or(NetworkReplayError::DnsLabelOutOfBounds)?;
        labels.push(
            std::str::from_utf8(label)
                .map_err(|_error| NetworkReplayError::DnsLabelNotUtf8)?
                .to_ascii_lowercase(),
        );
        cursor = label_end;
        if !jumped {
            next_offset = cursor;
        }
    }

    Ok((labels.join("."), next_offset))
}

fn dns_pointer_target(
    payload: &[u8],
    cursor: usize,
    label_len: u8,
) -> Result<usize, NetworkReplayError> {
    let low = *payload
        .get(cursor + 1)
        .ok_or(NetworkReplayError::DnsLabelOutOfBounds)?;
    let pointer = usize::from((u16::from(label_len & 0x3f) << 8) | u16::from(low));
    if pointer >= payload.len() {
        return Err(NetworkReplayError::DnsLabelOutOfBounds);
    }

    Ok(pointer)
}

fn dns_query_type(value: u16) -> DnsQueryType {
    match value {
        DNS_TYPE_A => DnsQueryType::A,
        DNS_TYPE_AAAA => DnsQueryType::Aaaa,
        other => DnsQueryType::Unknown(other),
    }
}

fn dns_record_data(record_type: u16, data: &[u8]) -> DnsRecordData {
    if record_type == DNS_TYPE_A && data.len() == IPV4_RDATA_LEN {
        return DnsRecordData::Ipv4Address(ipv4_text(data));
    }

    DnsRecordData::Raw {
        byte_len: data.len(),
    }
}

fn ipv4_text(bytes: &[u8]) -> String {
    format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
}
