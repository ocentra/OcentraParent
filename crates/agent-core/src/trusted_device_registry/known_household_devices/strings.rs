pub(super) fn merge_string_values(existing: &mut Vec<String>, incoming: Vec<String>) {
    for value in incoming {
        if !existing
            .iter()
            .any(|entry| entry.eq_ignore_ascii_case(&value))
        {
            existing.push(value);
        }
    }
}
