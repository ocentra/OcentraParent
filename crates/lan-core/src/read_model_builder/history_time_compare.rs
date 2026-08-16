pub(super) fn earliest_timestamp(first: Option<&str>, second: Option<&str>) -> Option<String> {
    match (first, second) {
        (Some(first), Some(second)) => {
            Some(if first <= second { first } else { second }.to_string())
        }
        (Some(first), None) => Some(first.to_string()),
        (None, Some(second)) => Some(second.to_string()),
        (None, None) => None,
    }
}

pub(super) fn latest_timestamp(first: Option<&str>, second: Option<&str>) -> Option<String> {
    match (first, second) {
        (Some(first), Some(second)) => {
            Some(if first >= second { first } else { second }.to_string())
        }
        (Some(first), None) => Some(first.to_string()),
        (None, Some(second)) => Some(second.to_string()),
        (None, None) => None,
    }
}
