mod validation;

pub(super) fn dns_like_name(payload: &[u8], offset: usize) -> Option<(String, usize)> {
    let mut labels = Vec::new();
    let mut cursor = offset;
    let mut next_offset = offset;
    let mut jumped = false;
    let mut jumps = 0_usize;
    loop {
        let label_len = *payload.get(cursor)?;
        if label_len == 0 {
            cursor += 1;
            if !jumped {
                next_offset = cursor;
            }
            break;
        }
        if let Some(pointer) = validation::pointer_at(payload, cursor, label_len)? {
            if !jumped {
                next_offset = cursor + 2;
            }
            cursor = pointer;
            jumped = true;
            jumps += 1;
            if jumps > 8 {
                return None;
            }
            continue;
        }
        if !validation::is_valid_label_len(label_len) {
            return None;
        }
        cursor += 1;
        let label_end = cursor.checked_add(usize::from(label_len))?;
        let label = payload.get(cursor..label_end)?;
        labels.push(String::from_utf8_lossy(label).to_string());
        cursor = label_end;
        if !jumped {
            next_offset = cursor;
        }
        if labels.len() > 16 {
            return None;
        }
    }
    Some((labels.join("."), next_offset))
}
