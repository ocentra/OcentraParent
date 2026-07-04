use super::{constants, NetworkReplayError};

pub(super) fn parse_dns_name(
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

        if label_len & constants::DNS_POINTER_MASK == constants::DNS_POINTER_VALUE {
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
            if jumps > constants::DNS_MAX_POINTER_JUMPS {
                return Err(NetworkReplayError::DnsNamePointerLoop);
            }
            continue;
        }
        if label_len & constants::DNS_POINTER_MASK != 0 {
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
