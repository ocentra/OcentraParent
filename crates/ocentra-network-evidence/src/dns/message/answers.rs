use super::{record, DnsResourceRecord, NetworkReplayError};

pub(super) fn parse_answers(
    payload: &[u8],
    mut offset: usize,
    answer_count: usize,
) -> Result<Vec<DnsResourceRecord>, NetworkReplayError> {
    let mut answers = Vec::new();
    for _ in 0..answer_count {
        let (answer, next_offset) = record::parse_resource_record(payload, offset)?;
        answers.push(answer);
        offset = next_offset;
    }

    Ok(answers)
}
