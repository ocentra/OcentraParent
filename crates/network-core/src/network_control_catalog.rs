#[path = "network_control_catalog_text.rs"]
mod network_control_catalog_text;

use self::network_control_catalog_text::{
    capitalize, clean_option_label, contains_any, lower_first, matrix_option_labels,
    searchable_text, split_explicit_options,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkControlKind {
    Toggle,
    SingleChoice,
    MultiChoice,
    Number,
    Duration,
    Schedule,
    RuleList,
    TargetList,
    Retention,
    ActionList,
    ReadOnlyStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkControlSelectionMode {
    Single,
    Multi,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkControlCardKind {
    SingleChoiceCompact,
    SingleChoiceMany,
    MultiChoiceNormal,
    MultiChoiceMany,
    Toggle,
    ScheduleCard,
    RuleListCard,
    TargetListCard,
    RetentionCard,
    StatusCard,
    NumberCard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkControlEffectStatus {
    AlreadyRepresented,
    NeedsEffectWiring,
    ManualRequired,
    Unavailable,
    FutureGap,
    Degraded,
    PermissionRequired,
    PermissionLimited,
    ProofRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkControlRuntimeOwner {
    PortalOnly,
    RustParentRuntime,
    AgentProtocol,
    RustService,
    ChildAgent,
    OsAdapter,
    ManualProof,
    ParentOwnedStorage,
    LocalAiRuntime,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkControlCapabilityState {
    Available,
    Disabled,
    Unsupported,
    PermissionRequired,
    PermissionLimited,
    Protected,
    Degraded,
    ManualRequired,
    FutureGap,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NetworkControlLayoutHints {
    pub preferred_column_span: usize,
    pub collapsible: bool,
    pub searchable_options: bool,
    pub option_group_count: usize,
    pub show_as_matrix_when_large: bool,
    pub show_selected_count: bool,
}

const POLICY_LANE_RETENTION_KEYWORDS: &[&str] = &[
    "retention",
    "custody",
    "audit",
    "journal",
    "deletion",
    "expiry",
    "redact",
    "export",
    "storage",
    "cache",
];
const POLICY_LANE_REPORT_KEYWORDS: &[&str] =
    &["report", "summary", "visible", "parent sees", "top "];
const POLICY_LANE_SCHEDULE_KEYWORDS: &[&str] = &[
    "budget",
    "schedule",
    "time window",
    "time budget",
    "network-active time",
    "bandwidth",
    "bytes",
    "connection-count",
];
const POLICY_LANE_APPROVAL_KEYWORDS: &[&str] =
    &["approval", "ask parent", "override", "parent approval"];
const POLICY_LANE_ENFORCEMENT_KEYWORDS: &[&str] = &[
    "block",
    "enforce",
    "firewall",
    "wfp",
    "packet filter",
    "vpn",
    "proxy",
    "tunnel",
    "adapter",
    "rollback",
    "strict action",
    "terminate",
    "router",
];
const POLICY_LANE_EVIDENCE_KEYWORDS: &[&str] = &[
    "evidence",
    "dns",
    "domain",
    "ip",
    "port",
    "protocol",
    "process",
    "flow",
    "metadata",
    "exact url",
    "encrypted",
    "https",
    "indicator",
    "attribution",
];
const POLICY_LANE_SETUP_KEYWORDS: &[&str] = &[
    "setup",
    "managed",
    "mdm",
    "entitlement",
    "permission",
    "profile",
    "service installation",
    "admin",
];

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

pub fn policy_lane_for(section_title: &str, group_title: &str, source_text: &str) -> &'static str {
    let searchable = searchable_text(&[section_title, group_title, source_text]);
    if contains_any(&searchable, POLICY_LANE_RETENTION_KEYWORDS) {
        if contains_any(&searchable, POLICY_LANE_REPORT_KEYWORDS) {
            return "reports";
        }
        return "audit";
    }
    if contains_any(&searchable, POLICY_LANE_SCHEDULE_KEYWORDS) {
        return "schedule";
    }
    if contains_any(&searchable, POLICY_LANE_APPROVAL_KEYWORDS) {
        return "approvals";
    }
    if contains_any(&searchable, POLICY_LANE_ENFORCEMENT_KEYWORDS) {
        return "enforcement";
    }
    if contains_any(&searchable, POLICY_LANE_EVIDENCE_KEYWORDS) {
        return "evidence";
    }
    if contains_any(&searchable, POLICY_LANE_SETUP_KEYWORDS) {
        return "setup";
    }
    "rules"
}

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
    if explicit_option_labels(source_text).is_empty() {
        NetworkControlKind::Toggle
    } else {
        NetworkControlKind::SingleChoice
    }
}

pub fn selection_mode_for(
    control_kind: NetworkControlKind,
    option_count: usize,
) -> NetworkControlSelectionMode {
    if matches!(
        control_kind,
        NetworkControlKind::MultiChoice
            | NetworkControlKind::ActionList
            | NetworkControlKind::TargetList
    ) {
        return NetworkControlSelectionMode::Multi;
    }
    if option_count > 4 && control_kind != NetworkControlKind::ReadOnlyStatus {
        return NetworkControlSelectionMode::Multi;
    }
    NetworkControlSelectionMode::Single
}

pub fn card_kind_for(
    control_kind: NetworkControlKind,
    selection_mode: NetworkControlSelectionMode,
    option_count: usize,
) -> NetworkControlCardKind {
    match control_kind {
        NetworkControlKind::Toggle => NetworkControlCardKind::Toggle,
        NetworkControlKind::Schedule => NetworkControlCardKind::ScheduleCard,
        NetworkControlKind::RuleList => NetworkControlCardKind::RuleListCard,
        NetworkControlKind::TargetList => NetworkControlCardKind::TargetListCard,
        NetworkControlKind::Retention => NetworkControlCardKind::RetentionCard,
        NetworkControlKind::ReadOnlyStatus => NetworkControlCardKind::StatusCard,
        NetworkControlKind::Number | NetworkControlKind::Duration => {
            NetworkControlCardKind::NumberCard
        }
        _ if selection_mode == NetworkControlSelectionMode::Multi => {
            if option_count > 4 {
                NetworkControlCardKind::MultiChoiceMany
            } else {
                NetworkControlCardKind::MultiChoiceNormal
            }
        }
        _ => {
            if option_count > 4 {
                NetworkControlCardKind::SingleChoiceMany
            } else {
                NetworkControlCardKind::SingleChoiceCompact
            }
        }
    }
}

pub fn layout_hints_for(
    control_kind: NetworkControlKind,
    selection_mode: NetworkControlSelectionMode,
    option_count: usize,
) -> NetworkControlLayoutHints {
    let many_options = option_count > 4;
    let list_like = selection_mode == NetworkControlSelectionMode::Multi
        || matches!(
            control_kind,
            NetworkControlKind::ActionList | NetworkControlKind::TargetList
        );
    NetworkControlLayoutHints {
        preferred_column_span: if many_options
            || matches!(
                control_kind,
                NetworkControlKind::Retention | NetworkControlKind::ReadOnlyStatus
            ) {
            2
        } else {
            1
        },
        collapsible: many_options
            || list_like
            || control_kind == NetworkControlKind::ReadOnlyStatus,
        searchable_options: many_options,
        option_group_count: if many_options {
            option_count.div_ceil(4)
        } else {
            1
        },
        show_as_matrix_when_large: many_options && list_like,
        show_selected_count: list_like,
    }
}

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

pub fn runtime_owner_for(
    section_title: &str,
    group_title: &str,
    source_text: &str,
) -> NetworkControlRuntimeOwner {
    let searchable = searchable_text(&[section_title, group_title, source_text]);
    if contains_any(
        &searchable,
        &[
            "portal ui",
            "portal authors",
            "does not run capture",
            "reports",
            "visible",
            "show ",
        ],
    ) {
        return NetworkControlRuntimeOwner::PortalOnly;
    }
    if contains_any(
        &searchable,
        &[
            "retention",
            "custody",
            "journal",
            "storage",
            "export",
            "cache",
            "audit",
            "redact",
        ],
    ) {
        return NetworkControlRuntimeOwner::ParentOwnedStorage;
    }
    if contains_any(&searchable, &["ai", "deterministic/ai", "local ai"]) {
        return NetworkControlRuntimeOwner::LocalAiRuntime;
    }
    if contains_any(
        &searchable,
        &[
            "firewall",
            "wfp",
            "packet filter",
            "vpn",
            "proxy",
            "router",
            "dns",
            "resolver",
            "network extension",
            "devicepolicymanager",
            "vpnservice",
            "adapter",
            "process",
            "endpoint",
            "etw",
            "ip helper",
        ],
    ) {
        return NetworkControlRuntimeOwner::OsAdapter;
    }
    if contains_any(
        &searchable,
        &[
            "policy", "rule", "compile", "decision", "fallback", "conflict", "proof",
        ],
    ) {
        return NetworkControlRuntimeOwner::RustParentRuntime;
    }
    NetworkControlRuntimeOwner::ChildAgent
}

pub fn capability_state_for(
    effect_status: NetworkControlEffectStatus,
) -> NetworkControlCapabilityState {
    match effect_status {
        NetworkControlEffectStatus::ManualRequired => NetworkControlCapabilityState::ManualRequired,
        NetworkControlEffectStatus::PermissionRequired => {
            NetworkControlCapabilityState::PermissionRequired
        }
        NetworkControlEffectStatus::PermissionLimited => {
            NetworkControlCapabilityState::PermissionLimited
        }
        NetworkControlEffectStatus::FutureGap => NetworkControlCapabilityState::FutureGap,
        NetworkControlEffectStatus::Degraded => NetworkControlCapabilityState::Degraded,
        NetworkControlEffectStatus::Unavailable => NetworkControlCapabilityState::Unavailable,
        NetworkControlEffectStatus::ProofRequired => NetworkControlCapabilityState::Protected,
        _ => NetworkControlCapabilityState::Available,
    }
}

pub fn capability_state_from_source_state(source_state: &str) -> NetworkControlCapabilityState {
    match source_state {
        "ready" => NetworkControlCapabilityState::Available,
        "ready-if-browser-capability-ready" => NetworkControlCapabilityState::Protected,
        "manual-required" => NetworkControlCapabilityState::ManualRequired,
        "not-implemented" => NetworkControlCapabilityState::Unavailable,
        "authoring-only" => NetworkControlCapabilityState::Disabled,
        _ => NetworkControlCapabilityState::Degraded,
    }
}

pub fn capability_requirement_for(
    section_title: &str,
    group_title: &str,
    source_text: &str,
) -> &'static str {
    let searchable = searchable_text(&[section_title, group_title, source_text]);
    if contains_any(
        &searchable,
        &[
            "exact url",
            "path/query",
            "active tab",
            "page title",
            "download source",
        ],
    ) {
        return "managed-browser-or-explicit-url-filter-proof";
    }
    if contains_any(&searchable, &["domain", "dns", "resolver"]) {
        return "dns-or-domain-attribution-source-with-confidence";
    }
    if contains_any(
        &searchable,
        &["ip", "port", "protocol", "flow", "endpoint", "process"],
    ) {
        return "local-network-flow-metadata-evidence";
    }
    if contains_any(
        &searchable,
        &[
            "block",
            "firewall",
            "wfp",
            "packet filter",
            "vpn",
            "router",
            "strict",
            "enforcement",
        ],
    ) {
        return "real-platform-network-adapter-proof";
    }
    if contains_any(
        &searchable,
        &["retention", "custody", "report", "audit", "storage"],
    ) {
        return "local-first-custody-and-retention-policy";
    }
    "network-control-capability-registry"
}

pub fn proof_requirement_for(
    section_title: &str,
    group_title: &str,
    source_text: &str,
) -> Option<&'static str> {
    let searchable = searchable_text(&[section_title, group_title, source_text]);
    if contains_any(
        &searchable,
        &[
            "exact url",
            "path/query",
            "active tab",
            "page title",
            "download source",
        ],
    ) {
        return Some(
            "Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.",
        );
    }
    if contains_any(
        &searchable,
        &[
            "decrypted",
            "payload",
            "page body",
            "chat content",
            "search terms",
            "form values",
            "cookies",
            "tokens",
            "credentials",
            "packet payload",
        ],
    ) {
        return Some(
            "Network controls must not collect decrypted content or payload fields; use metadata evidence only.",
        );
    }
    if contains_any(
        &searchable,
        &[
            "block",
            "firewall",
            "wfp",
            "packet filter",
            "vpn",
            "router",
            "always-on",
            "lockdown",
            "force all traffic",
            "strict",
        ],
    ) {
        return Some(
            "Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.",
        );
    }
    if contains_any(
        &searchable,
        &[
            "domain",
            "dns",
            "ip",
            "port",
            "protocol",
            "flow",
            "process",
            "indicator",
            "attribution",
            "confidence",
            "evidence id",
        ],
    ) {
        return Some(
            "Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.",
        );
    }
    None
}

pub fn fallback_for(effect_status: NetworkControlEffectStatus, source_text: &str) -> &'static str {
    let searchable = source_text.to_lowercase();
    if contains_any(
        &searchable,
        &["exact url", "path/query", "active tab", "page title"],
    ) {
        return "Hide or disable exact URL controls unless managed browser, explicit URL filter, or proxy proof exists.";
    }
    if contains_any(
        &searchable,
        &[
            "decrypted",
            "payload",
            "page body",
            "chat content",
            "search terms",
            "form values",
            "cookies",
            "tokens",
            "credentials",
        ],
    ) {
        return "Never collect decrypted content or sensitive payload fields in the network-control catalog.";
    }
    match effect_status {
        NetworkControlEffectStatus::ManualRequired => {
            "Show manual-required until setup, privileges, and adapter proof exist; compile observe or unavailable fallback."
        }
        NetworkControlEffectStatus::Degraded => {
            "Render degraded state and keep unsupported behavior out of compiled enforcement plans."
        }
        NetworkControlEffectStatus::ProofRequired => {
            "Require evidence proof before strict effect; otherwise fall back to observe, warn, ask, or unavailable."
        }
        NetworkControlEffectStatus::FutureGap => {
            "Expose as future or planning-only and do not claim current runtime behavior."
        }
        _ => "Portal renders the control; child-agent/runtime ownership remains explicit.",
    }
}

pub fn helper_text_for(section_title: &str, group_title: &str, source_text: &str) -> String {
    proof_requirement_for(section_title, group_title, source_text)
        .unwrap_or_else(|| capability_requirement_for(section_title, group_title, source_text))
        .to_owned()
}

pub fn question_from_source_text(source_text: &str, explicit_question: Option<&str>) -> String {
    if let Some(question) = explicit_question {
        if !question.is_empty() {
            return question.to_owned();
        }
    }
    let trimmed = source_text.trim_end_matches('.');
    if trimmed.ends_with('?') {
        return trimmed.to_owned();
    }
    if trimmed.starts_with("Capability matrix row |") {
        let capability = trimmed
            .split(" | ")
            .find_map(|part| part.strip_prefix("Capability="))
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or("network capability");
        return format!("Represent {capability} capability status.");
    }
    if let Some(colon_index) = trimmed.find(':') {
        return format!("Configure {}.", trimmed[..colon_index].to_lowercase());
    }
    format!("Represent {}?", lower_first(trimmed))
}

pub fn explicit_option_labels(source_text: &str) -> Vec<String> {
    let matrix_options = matrix_option_labels(source_text);
    if !matrix_options.is_empty() {
        return matrix_options;
    }
    let Some(colon_index) = source_text.find(':') else {
        return Vec::new();
    };
    split_explicit_options(&source_text[colon_index + 1..source_text.len()])
        .into_iter()
        .map(|part| clean_option_label(&part))
        .filter(|part| !part.is_empty())
        .collect()
}

pub fn slug_token(value: &str) -> String {
    let mut slugged = String::new();
    let mut previous_dash = false;
    for ch in value.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() {
            slugged.push(ch);
            previous_dash = false;
        } else if !previous_dash {
            slugged.push('-');
            previous_dash = true;
        }
    }
    let slugged = slugged.trim_matches('-').to_owned();
    if slugged.is_empty() {
        "item".to_owned()
    } else {
        slugged
    }
}

pub fn title_from_token(value: &str) -> String {
    value
        .split('-')
        .filter(|part| !part.is_empty())
        .map(capitalize)
        .collect::<Vec<_>>()
        .join(" ")
}
