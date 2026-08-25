use super::hex_digest;

pub(super) fn state_valid(
    state: &str,
    owner_receipt: Option<&str>,
    owner_transition: Option<&str>,
) -> bool {
    match state {
        "owner-approval-required" | "approved" | "revoked" => {
            owner_receipt.is_none() && owner_transition.is_none()
        }
        "completed" => {
            owner_receipt.is_some_and(hex_digest)
                && owner_transition.is_some_and(|value| !value.trim().is_empty())
        }
        _ => false,
    }
}

pub(super) fn effect_matches_kind(kind: &str, effect: i64) -> bool {
    matches!(
        (kind, effect),
        ("forgot-login", 1)
            | ("lost-parent-device", 2)
            | ("compromised-account", 2)
            | ("child-reinstall", 3)
            | ("household-transfer", 4)
    )
}
