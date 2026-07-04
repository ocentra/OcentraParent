use super::super::super::text::sanitize_mdns_text;
use super::super::super::MdnsDnsSdTxtRecord;

pub(super) fn parse_txt_records(data: &[u8]) -> Vec<MdnsDnsSdTxtRecord> {
    let mut offset = 0;
    let mut records = Vec::new();
    while offset < data.len() {
        let len = match data.get(offset) {
            Some(value) => usize::from(*value),
            None => break,
        };
        offset += 1;
        let Some(end) = offset.checked_add(len) else {
            break;
        };
        if offset > data.len() || end > data.len() {
            break;
        }
        push_txt_record_entry(&mut records, &data[offset..end]);
        offset = end;
    }
    records
}

pub(super) fn push_txt_record_entry(records: &mut Vec<MdnsDnsSdTxtRecord>, entry: &[u8]) {
    let entry = std::string::String::from_utf8_lossy(entry).to_string();
    if entry.is_empty() {
        return;
    }
    let (key, value) = split_txt_record_entry(&entry);
    if let Some(key) = key {
        records.push(MdnsDnsSdTxtRecord { key, value });
    }
}

fn split_txt_record_entry(entry: &str) -> (Option<String>, Option<String>) {
    if let Some((key, value)) = entry.split_once('=') {
        let key = sanitize_mdns_text(key);
        let value = sanitize_mdns_text(value).filter(|value| !value.is_empty());
        (key, value)
    } else {
        (sanitize_mdns_text(entry), None)
    }
}
