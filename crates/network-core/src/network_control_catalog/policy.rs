use super::network_control_catalog_text::{contains_any, searchable_text};

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
