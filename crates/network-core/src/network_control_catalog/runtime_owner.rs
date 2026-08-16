use super::network_control_catalog_text::{contains_any, searchable_text};
use super::NetworkControlRuntimeOwner;

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
