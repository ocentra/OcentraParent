use super::network_control_catalog_text::{contains_any, searchable_text};
use super::NetworkControlEffectStatus;

const EFFECT_STATUS_FUTURE_KEYWORDS: &[&str] =
    &["future", "later", "not yet", "planned", "missing", "gap"];
const EFFECT_STATUS_MANUAL_KEYWORDS: &[&str] = &[
    "manual-required",
    "manual required",
    "admin",
    "privilege",
    "service installation",
    "driver",
    "mdm",
    "supervision",
    "entitlement",
    "router api",
    "wfp",
    "windows filtering platform",
    "always-on",
    "lockdown",
    "force all traffic",
];
const EFFECT_STATUS_PERMISSION_KEYWORDS: &[&str] = &[
    "permission",
    "profile",
    "tcc",
    "protected",
    "review",
    "signing",
];
const EFFECT_STATUS_DEGRADED_KEYWORDS: &[&str] = &[
    "limited",
    "partial",
    "varies",
    "ambiguous",
    "stale",
    "unavailable",
    "unsupported",
    "bypass",
    "miss",
    "cannot",
    "usually cannot",
];
const EFFECT_STATUS_PROOF_KEYWORDS: &[&str] = &[
    "exact url",
    "path/query",
    "https",
    "decrypted",
    "payload",
    "page body",
    "chat content",
    "search terms",
    "form values",
    "cookies",
    "tokens",
    "credentials",
    "proof",
    "evidence id",
    "confidence",
    "must cite",
    "not proof",
];
const EFFECT_STATUS_ALREADY_REPRESENTED_KEYWORDS: &[&str] = &[
    "retention",
    "custody",
    "report",
    "redact",
    "local-first",
    "audit",
    "summary",
    "show ",
];

pub fn effect_status_for(
    section_title: &str,
    group_title: &str,
    source_text: &str,
) -> NetworkControlEffectStatus {
    let searchable = searchable_text(&[section_title, group_title, source_text]);
    let source_searchable = source_text.to_lowercase();
    if contains_any(&source_searchable, EFFECT_STATUS_FUTURE_KEYWORDS) {
        return NetworkControlEffectStatus::FutureGap;
    }
    if contains_any(&searchable, EFFECT_STATUS_MANUAL_KEYWORDS) {
        return NetworkControlEffectStatus::ManualRequired;
    }
    if contains_any(&source_searchable, EFFECT_STATUS_PERMISSION_KEYWORDS)
        || (source_searchable.contains("user") && source_searchable.contains("setup"))
    {
        return NetworkControlEffectStatus::PermissionRequired;
    }
    if contains_any(&searchable, EFFECT_STATUS_DEGRADED_KEYWORDS) {
        return NetworkControlEffectStatus::Degraded;
    }
    if contains_any(&searchable, EFFECT_STATUS_PROOF_KEYWORDS) {
        return NetworkControlEffectStatus::ProofRequired;
    }
    if contains_any(&searchable, EFFECT_STATUS_ALREADY_REPRESENTED_KEYWORDS) {
        return NetworkControlEffectStatus::AlreadyRepresented;
    }
    NetworkControlEffectStatus::NeedsEffectWiring
}
