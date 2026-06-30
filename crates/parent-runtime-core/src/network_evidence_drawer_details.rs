use super::*;

pub(super) fn joined_detail(values: impl IntoIterator<Item = Option<String>>) -> String {
    let normalized = values.into_iter().flatten().collect::<Vec<_>>();
    if normalized.is_empty() {
        return not_reported();
    }
    normalized.join(EVENT_DETAIL_SEPARATOR)
}

pub(super) fn detail_from_optional_str(value: Option<&str>) -> String {
    match value.map(str::trim).filter(|candidate| !candidate.is_empty()) {
        Some(candidate) => candidate.to_string(),
        None => not_reported(),
    }
}

pub(super) fn not_reported() -> String {
    NOT_REPORTED.to_string()
}
