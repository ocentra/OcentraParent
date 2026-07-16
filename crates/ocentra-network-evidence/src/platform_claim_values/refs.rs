pub(super) fn compact_refs(values: Vec<Option<String>>) -> Vec<String> {
    let mut refs = Vec::new();
    for value in values.into_iter().flatten() {
        if !value.is_empty() && !refs.contains(&value) {
            refs.push(value);
        }
    }
    refs
}
