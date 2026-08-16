use super::network_control_catalog_text::contains_any;
use super::NetworkControlKind;

pub fn control_kind_for(source_text: &str, explicit_kind: Option<&str>) -> NetworkControlKind {
    match explicit_kind {
        Some("boolean") => return NetworkControlKind::Toggle,
        Some("single-choice") => return NetworkControlKind::SingleChoice,
        Some("multi-choice") => return NetworkControlKind::MultiChoice,
        _ => {}
    }
    if source_text.starts_with("Capability matrix row |") {
        return NetworkControlKind::ReadOnlyStatus;
    }
    let searchable = source_text.to_lowercase();
    if contains_any(
        &searchable,
        &[
            "budget",
            "bytes",
            "count",
            "minutes",
            "duration",
            "timer",
            "threshold",
            "retention",
        ],
    ) {
        if contains_any(&searchable, &["retention", "custody", "deletion", "expiry"]) {
            return NetworkControlKind::Retention;
        }
        return NetworkControlKind::Number;
    }
    if contains_any(&searchable, &["schedule", "time window"]) {
        return NetworkControlKind::Schedule;
    }
    if contains_any(
        &searchable,
        &[
            "allow",
            "block",
            "warn",
            "ask",
            "enforce",
            "terminate",
            "force",
            "route",
            "adapter",
            "actions",
        ],
    ) {
        return NetworkControlKind::ActionList;
    }
    if contains_any(
        &searchable,
        &[
            "target",
            "domain",
            "ip",
            "port",
            "protocol",
            "process",
            "exception",
            "indicator",
            "source",
            "field",
            "scope",
            "category",
        ],
    ) {
        return NetworkControlKind::MultiChoice;
    }
    if super::question::explicit_option_labels(source_text).is_empty() {
        NetworkControlKind::Toggle
    } else {
        NetworkControlKind::SingleChoice
    }
}
