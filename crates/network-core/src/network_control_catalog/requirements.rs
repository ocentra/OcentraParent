use super::network_control_catalog_text::contains_any;

pub fn capability_requirement_for(
    section_title: &str,
    group_title: &str,
    source_text: &str,
) -> &'static str {
    let searchable = super::network_control_catalog_text::searchable_text(&[
        section_title,
        group_title,
        source_text,
    ]);
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

pub fn helper_text_for(section_title: &str, group_title: &str, source_text: &str) -> String {
    super::proof_requirement::proof_requirement_for(section_title, group_title, source_text)
        .unwrap_or_else(|| capability_requirement_for(section_title, group_title, source_text))
        .to_owned()
}
