use super::{
    DnsMessage, DnsQueryType, DnsQuestion, DnsRecordData, DnsResourceRecord, NetworkReplayError,
};

mod answers;
mod constants;
mod data;
mod name;
mod questions;
mod record;

pub fn parse_dns_message(payload: &[u8]) -> Result<DnsMessage, NetworkReplayError> {
    if payload.len() < constants::DNS_HEADER_LEN {
        return Err(NetworkReplayError::DnsPacketTooShort);
    }

    let transaction_id = u16::from_be_bytes([payload[0], payload[1]]);
    let flags = u16::from_be_bytes([payload[2], payload[3]]);
    let question_count = u16::from_be_bytes([payload[4], payload[5]]);
    let answer_count = u16::from_be_bytes([payload[6], payload[7]]);
    if question_count == 0 {
        return Err(NetworkReplayError::DnsQuestionMissing);
    }

    let (questions, offset) = questions::parse_questions(payload, usize::from(question_count))?;
    let answers = answers::parse_answers(payload, offset, usize::from(answer_count))?;
    Ok(DnsMessage {
        transaction_id,
        is_response: flags & constants::DNS_RESPONSE_FLAG != 0,
        questions,
        answers,
    })
}
