use super::PersistedIntentRow;

pub(super) fn challenge_lifecycle_matches(row: &PersistedIntentRow) -> bool {
    match (
        row.lifecycle_state.as_str(),
        row.registration_state.as_str(),
    ) {
        ("issued", "pending") => row.challenge_lifecycle_state == "issued",
        ("consumed", "pending" | "completed") => row.challenge_lifecycle_state == "consumed",
        _ => false,
    }
}
