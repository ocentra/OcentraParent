pub(super) fn merge(existing: &mut String, incoming: &str) {
    if incoming.is_empty() {
        return;
    }
    if existing.is_empty() || incoming < existing.as_str() {
        *existing = incoming.to_string();
    }
}
