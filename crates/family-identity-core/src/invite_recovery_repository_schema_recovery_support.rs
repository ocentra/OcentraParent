pub(super) fn valid(
    kind: &str,
    channel: &str,
    id: Option<&str>,
    issuer: Option<&str>,
    scope: Option<&str>,
    expires: Option<i64>,
) -> bool {
    match channel {
        "support-assisted" => {
            nonempty(id)
                && nonempty(issuer)
                && scope_matches_kind(kind, scope)
                && expires.is_some_and(|value| value > 0)
        }
        "self-serve" | "household-owner-assisted" => {
            id.is_none() && issuer.is_none() && scope.is_none() && expires.is_none()
        }
        _ => false,
    }
}

fn nonempty(value: Option<&str>) -> bool {
    value.is_some_and(|value| !value.trim().is_empty())
}

fn scope_matches_kind(kind: &str, scope: Option<&str>) -> bool {
    match kind {
        "household-transfer" => scope == Some("household"),
        "lost-parent-device" | "compromised-account" | "child-reinstall" => {
            scope == Some("device-control")
        }
        "forgot-login" => matches!(scope, Some("household") | Some("device-control")),
        _ => false,
    }
}
