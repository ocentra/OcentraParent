use std::collections::HashSet;

use super::super::{MDNS_MAX_LABELS, MDNS_MAX_POINTER_JUMPS};

pub(super) fn parse_dns_name(payload: &[u8], offset: usize) -> Option<(String, usize)> {
    let mut labels = Vec::new();
    let mut state = DnsNameParseState::new(offset);

    loop {
        if labels.len() >= MDNS_MAX_LABELS {
            return None;
        }
        let label_len = *payload.get(state.cursor)?;
        if label_len == 0 {
            state.finish_current_label();
            break;
        }
        if is_compression_pointer(label_len) {
            state.follow_pointer(payload, label_len)?;
            continue;
        }
        state.push_label(payload, label_len, &mut labels)?;
    }

    Some((labels.join("."), state.next_offset))
}

struct DnsNameParseState {
    cursor: usize,
    next_offset: usize,
    jumped: bool,
    jumps: usize,
    visited: HashSet<usize>,
}

impl DnsNameParseState {
    fn new(offset: usize) -> Self {
        let mut visited = HashSet::new();
        visited.insert(offset);
        Self {
            cursor: offset,
            next_offset: offset,
            jumped: false,
            jumps: 0,
            visited,
        }
    }

    fn finish_current_label(&mut self) {
        self.cursor += 1;
        if !self.jumped {
            self.next_offset = self.cursor;
        }
    }

    fn follow_pointer(&mut self, payload: &[u8], label_len: u8) -> Option<()> {
        let low = *payload.get(self.cursor + 1)?;
        let pointer = usize::from((u16::from(label_len & 0x3f) << 8) | u16::from(low));
        if pointer >= payload.len() || !self.visited.insert(pointer) {
            return None;
        }
        if !self.jumped {
            self.next_offset = self.cursor + 2;
        }
        self.cursor = pointer;
        self.jumped = true;
        self.jumps += 1;
        (self.jumps <= MDNS_MAX_POINTER_JUMPS).then_some(())
    }

    fn push_label(
        &mut self,
        payload: &[u8],
        label_len: u8,
        labels: &mut Vec<String>,
    ) -> Option<()> {
        if invalid_label_len(label_len) {
            return None;
        }
        self.cursor += 1;
        let label_end = self.cursor.checked_add(usize::from(label_len))?;
        let label = payload.get(self.cursor..label_end)?;
        labels.push(String::from_utf8_lossy(label).to_string());
        self.cursor = label_end;
        if !self.jumped {
            self.next_offset = self.cursor;
        }
        Some(())
    }
}

fn is_compression_pointer(label_len: u8) -> bool {
    label_len & 0b1100_0000 == 0b1100_0000
}

fn invalid_label_len(label_len: u8) -> bool {
    label_len == 0 || label_len > 63 || label_len & 0b1100_0000 != 0
}
