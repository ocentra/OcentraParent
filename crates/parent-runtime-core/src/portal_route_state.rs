#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParentPortalPageMode {
    ParentOverview,
    ParentManage,
    ParentGuide,
}

impl ParentPortalPageMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParentOverview => "parentOverview",
            Self::ParentManage => "parentManage",
            Self::ParentGuide => "parentGuide",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParentPortalManageLane {
    Portal,
    ChildPolicy,
    DeviceOps,
}

impl ParentPortalManageLane {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Portal => "portal",
            Self::ChildPolicy => "childPolicy",
            Self::DeviceOps => "deviceOps",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParentPortalServiceReachability {
    Reachable,
    Degraded,
    Unavailable,
}

impl ParentPortalServiceReachability {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reachable => "reachable",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParentPortalServiceDegradationReasonCode {
    MissingSnapshotRows,
    Connecting,
    StaleSnapshotRows,
    ServiceUnavailable,
}

impl ParentPortalServiceDegradationReasonCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingSnapshotRows => "missing-snapshot-rows",
            Self::Connecting => "connecting",
            Self::StaleSnapshotRows => "stale-snapshot-rows",
            Self::ServiceUnavailable => "service-unavailable",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParentPortalRouteState {
    pub page_mode: ParentPortalPageMode,
    pub selected_control_id: &'static str,
    pub manage_lane: Option<ParentPortalManageLane>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParentPortalServiceReachabilityState {
    pub service_reachability: ParentPortalServiceReachability,
    pub service_degradation_reason_code: Option<ParentPortalServiceDegradationReasonCode>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParentPortalRouteStateRecord {
    route: &'static str,
    page_mode: ParentPortalPageMode,
    selected_control_id: &'static str,
    manage_lane: Option<ParentPortalManageLane>,
}

const ROUTE_STATE_RECORDS: &[ParentPortalRouteStateRecord] = &[
    route_state("overview", ParentPortalPageMode::ParentOverview, "activity-store", None),
    route_state("assistant", ParentPortalPageMode::ParentGuide, "ai-runtime", None),
    route_state("start", ParentPortalPageMode::ParentOverview, "setup-overall", None),
    route_state(
        "activity",
        ParentPortalPageMode::ParentManage,
        "reports-settings",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "browser",
        ParentPortalPageMode::ParentManage,
        "managed-web",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "browser-settings",
        ParentPortalPageMode::ParentManage,
        "browser-settings",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state("policy", ParentPortalPageMode::ParentGuide, "rules-policy", None),
    route_state(
        "policy-apps",
        ParentPortalPageMode::ParentManage,
        "policy-apps",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "policy-games",
        ParentPortalPageMode::ParentManage,
        "policy-games",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "policy-screen",
        ParentPortalPageMode::ParentManage,
        "screen-analysis",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "policy-network",
        ParentPortalPageMode::ParentManage,
        "network-activity",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "policy-tracking",
        ParentPortalPageMode::ParentManage,
        "policy-tracking",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "policy-remote-screen",
        ParentPortalPageMode::ParentManage,
        "policy-remote-screen",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "rule-management",
        ParentPortalPageMode::ParentManage,
        "rules-management",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "schedules",
        ParentPortalPageMode::ParentManage,
        "schedules-budgets",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "approvals",
        ParentPortalPageMode::ParentManage,
        "approvals",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "enforcement",
        ParentPortalPageMode::ParentManage,
        "enforcement-readiness",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "privacy-design",
        ParentPortalPageMode::ParentGuide,
        "privacy-design",
        None,
    ),
    route_state(
        "memory",
        ParentPortalPageMode::ParentGuide,
        "memory-citations",
        None,
    ),
    route_state(
        "memory-settings",
        ParentPortalPageMode::ParentManage,
        "memory-settings",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "ai-guide",
        ParentPortalPageMode::ParentGuide,
        "local-ai-evidence",
        None,
    ),
    route_state(
        "ai-runtime",
        ParentPortalPageMode::ParentManage,
        "ai-runtime",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "api-providers",
        ParentPortalPageMode::ParentManage,
        "api-providers",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "reports-guide",
        ParentPortalPageMode::ParentGuide,
        "reports-summaries",
        None,
    ),
    route_state(
        "screen-analysis",
        ParentPortalPageMode::ParentManage,
        "reports-settings",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "app-game-sessions",
        ParentPortalPageMode::ParentManage,
        "app-game-sessions",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "network-activity",
        ParentPortalPageMode::ParentManage,
        "reports-settings",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "devices",
        ParentPortalPageMode::ParentManage,
        "lan-pairing",
        Some(ParentPortalManageLane::DeviceOps),
    ),
    route_state(
        "lan-pairing",
        ParentPortalPageMode::ParentManage,
        "lan-pairing",
        Some(ParentPortalManageLane::DeviceOps),
    ),
    route_state(
        "capability-status",
        ParentPortalPageMode::ParentManage,
        "lan-pairing",
        Some(ParentPortalManageLane::DeviceOps),
    ),
    route_state(
        "notifications",
        ParentPortalPageMode::ParentManage,
        "notifications",
        Some(ParentPortalManageLane::Portal),
    ),
    route_state(
        "notification-channels",
        ParentPortalPageMode::ParentManage,
        "notification-channels",
        Some(ParentPortalManageLane::Portal),
    ),
    route_state(
        "drive-connections",
        ParentPortalPageMode::ParentManage,
        "drive-exports",
        Some(ParentPortalManageLane::Portal),
    ),
    route_state(
        "export-retention",
        ParentPortalPageMode::ParentManage,
        "export-retention",
        Some(ParentPortalManageLane::Portal),
    ),
    route_state(
        "remote-access",
        ParentPortalPageMode::ParentManage,
        "remote-access",
        Some(ParentPortalManageLane::DeviceOps),
    ),
    route_state(
        "report-compiler",
        ParentPortalPageMode::ParentManage,
        "reports-settings",
        Some(ParentPortalManageLane::ChildPolicy),
    ),
    route_state(
        "audit-history",
        ParentPortalPageMode::ParentManage,
        "audit-history",
        Some(ParentPortalManageLane::Portal),
    ),
    route_state(
        "subscription",
        ParentPortalPageMode::ParentManage,
        "subscription-plans",
        Some(ParentPortalManageLane::Portal),
    ),
    route_state(
        "entitlements",
        ParentPortalPageMode::ParentManage,
        "entitlements",
        Some(ParentPortalManageLane::Portal),
    ),
    route_state(
        "platforms-install",
        ParentPortalPageMode::ParentManage,
        "lan-pairing",
        Some(ParentPortalManageLane::DeviceOps),
    ),
    route_state(
        "install-updates",
        ParentPortalPageMode::ParentManage,
        "lan-pairing",
        Some(ParentPortalManageLane::DeviceOps),
    ),
    route_state(
        "diagnostics",
        ParentPortalPageMode::ParentManage,
        "support-api-status",
        Some(ParentPortalManageLane::Portal),
    ),
    route_state(
        "proof-panels",
        ParentPortalPageMode::ParentManage,
        "dev-proof-panels",
        None,
    ),
    route_state(
        "settings-rules",
        ParentPortalPageMode::ParentManage,
        "family-settings",
        Some(ParentPortalManageLane::Portal),
    ),
    route_state(
        "app-layout",
        ParentPortalPageMode::ParentManage,
        "app-layout",
        None,
    ),
    route_state(
        "frame-tuner",
        ParentPortalPageMode::ParentManage,
        "app-layout",
        None,
    ),
    route_state(
        "commands",
        ParentPortalPageMode::ParentManage,
        "dev-commands",
        None,
    ),
    route_state("events", ParentPortalPageMode::ParentManage, "dev-events", None),
    route_state("logs", ParentPortalPageMode::ParentManage, "dev-logs", None),
];

const fn route_state(
    route: &'static str,
    page_mode: ParentPortalPageMode,
    selected_control_id: &'static str,
    manage_lane: Option<ParentPortalManageLane>,
) -> ParentPortalRouteStateRecord {
    ParentPortalRouteStateRecord {
        route,
        page_mode,
        selected_control_id,
        manage_lane,
    }
}

fn route_state_record(route: &str) -> Option<&'static ParentPortalRouteStateRecord> {
    ROUTE_STATE_RECORDS.iter().find(|record| record.route == route)
}

pub fn portal_route_from_hash_path(route_hash: &str) -> Option<&'static str> {
    let normalized_hash = route_hash
        .strip_prefix("#/")
        .or_else(|| route_hash.strip_prefix('#'))
        .unwrap_or(route_hash);
    let route = normalized_hash.split('?').next().unwrap_or_default();
    route_state_record(route).map(|record| record.route)
}

pub fn parent_portal_route_state(route: &str) -> Option<ParentPortalRouteState> {
    route_state_record(route).map(|record| ParentPortalRouteState {
        page_mode: record.page_mode,
        selected_control_id: record.selected_control_id,
        manage_lane: record.manage_lane,
    })
}

pub fn parent_portal_manage_lane_for_route(route: &str) -> Option<ParentPortalManageLane> {
    parent_portal_route_state(route).and_then(|record| record.manage_lane)
}

pub fn resolve_parent_portal_service_reachability(
    connection_state: &str,
    has_snapshot_rows: bool,
) -> ParentPortalServiceReachabilityState {
    match connection_state {
        "connected" if has_snapshot_rows => ParentPortalServiceReachabilityState {
            service_reachability: ParentPortalServiceReachability::Reachable,
            service_degradation_reason_code: None,
        },
        "connected" => ParentPortalServiceReachabilityState {
            service_reachability: ParentPortalServiceReachability::Degraded,
            service_degradation_reason_code: Some(
                ParentPortalServiceDegradationReasonCode::MissingSnapshotRows,
            ),
        },
        "connecting" => ParentPortalServiceReachabilityState {
            service_reachability: ParentPortalServiceReachability::Degraded,
            service_degradation_reason_code: Some(
                ParentPortalServiceDegradationReasonCode::Connecting,
            ),
        },
        _ if has_snapshot_rows => ParentPortalServiceReachabilityState {
            service_reachability: ParentPortalServiceReachability::Degraded,
            service_degradation_reason_code: Some(
                ParentPortalServiceDegradationReasonCode::StaleSnapshotRows,
            ),
        },
        _ => ParentPortalServiceReachabilityState {
            service_reachability: ParentPortalServiceReachability::Unavailable,
            service_degradation_reason_code: Some(
                ParentPortalServiceDegradationReasonCode::ServiceUnavailable,
            ),
        },
    }
}

pub fn portal_route_state_typescript() -> String {
    let mut output = String::from(
        "/* generated from crates/parent-runtime-core/src/portal_route_state.rs */\n\n",
    );
    output.push_str(
        "export type GeneratedParentPortalPageMode = 'parentOverview' | 'parentManage' | 'parentGuide';\n",
    );
    output.push_str(
        "export type GeneratedParentPortalManageLane = 'portal' | 'childPolicy' | 'deviceOps';\n",
    );
    output.push_str(
        "export type GeneratedParentPortalServiceConnectionState = 'connected' | 'connecting' | 'disconnected' | 'error';\n",
    );
    output.push_str(
        "export type GeneratedParentPortalServiceReachability = 'reachable' | 'degraded' | 'unavailable';\n",
    );
    output.push_str("export type GeneratedParentPortalServiceDegradationReasonCode =\n");
    for reason_code in [
        ParentPortalServiceDegradationReasonCode::MissingSnapshotRows,
        ParentPortalServiceDegradationReasonCode::Connecting,
        ParentPortalServiceDegradationReasonCode::StaleSnapshotRows,
        ParentPortalServiceDegradationReasonCode::ServiceUnavailable,
    ] {
        output.push_str(&format!("  | '{}'\n", reason_code.as_str()));
    }
    output.push_str(";\n\n");
    output.push_str("export type GeneratedParentPortalRouteStateRecord = {\n");
    output.push_str("  readonly pageMode: GeneratedParentPortalPageMode;\n");
    output.push_str("  readonly selectedControlId: string;\n");
    output.push_str("  readonly manageLane: GeneratedParentPortalManageLane | null;\n");
    output.push_str("};\n\n");
    output.push_str("export type GeneratedParentPortalServiceReachabilityState = {\n");
    output.push_str(
        "  readonly serviceReachability: GeneratedParentPortalServiceReachability;\n",
    );
    output.push_str(
        "  readonly serviceDegradationReasonCode: GeneratedParentPortalServiceDegradationReasonCode | null;\n",
    );
    output.push_str("};\n\n");
    output.push_str(&portal_route_state_records_typescript());
    output.push_str(&portal_route_state_service_reachability_typescript());
    output
}

fn portal_route_state_records_typescript() -> String {
    let mut output = String::new();
    output.push_str("const portalRouteStateRecords = {\n");
    for record in ROUTE_STATE_RECORDS {
        let manage_lane = match record.manage_lane {
            Some(lane) => format!("'{}'", lane.as_str()),
            None => "null".to_string(),
        };
        output.push_str(&format!(
            "  '{}': {{ pageMode: '{}', selectedControlId: '{}', manageLane: {} }},\n",
            record.route,
            record.page_mode.as_str(),
            record.selected_control_id,
            manage_lane
        ));
    }
    output.push_str("} as const satisfies Record<string, GeneratedParentPortalRouteStateRecord>;\n\n");
    output.push_str("const portalRouteStateRecordKeys = Object.keys(portalRouteStateRecords);\n");
    output.push_str("const portalRouteStateRecordKeySet = new Set<string>(portalRouteStateRecordKeys);\n\n");
    output.push_str("export function generatedPortalRouteFromHashPath(routeHash: string): string | null {\n");
    output.push_str("  const normalizedHash = routeHash.replace(/^#\\/?/u, '');\n");
    output.push_str("  const route = normalizedHash.split('?')[0] ?? '';\n");
    output.push_str("  return portalRouteStateRecordKeySet.has(route) ? route : null;\n");
    output.push_str("}\n\n");
    output.push_str("export function generatedParentPortalRouteState(route: string): GeneratedParentPortalRouteStateRecord | null {\n");
    output.push_str("  return Object.prototype.hasOwnProperty.call(portalRouteStateRecords, route)\n");
    output.push_str(
        "    ? portalRouteStateRecords[route as keyof typeof portalRouteStateRecords]\n",
    );
    output.push_str("    : null;\n");
    output.push_str("}\n\n");
    output.push_str("export function generatedParentPortalManageLaneForRoute(route: string): GeneratedParentPortalManageLane | null {\n");
    output.push_str("  return generatedParentPortalRouteState(route)?.manageLane ?? null;\n");
    output.push_str("}\n");
    output
}

fn portal_route_state_service_reachability_typescript() -> String {
    let mut output = String::new();
    output.push_str(
        "export function generatedResolveParentPortalServiceReachability(\n",
    );
    output.push_str("  connectionState: GeneratedParentPortalServiceConnectionState,\n");
    output.push_str("  hasSnapshotRows: boolean\n");
    output.push_str("): GeneratedParentPortalServiceReachabilityState {\n");
    output.push_str("  if (connectionState === 'connected') {\n");
    output.push_str("    return hasSnapshotRows\n");
    output.push_str(
        "      ? { serviceReachability: 'reachable', serviceDegradationReasonCode: null }\n",
    );
    output.push_str("      : {\n");
    output.push_str("          serviceReachability: 'degraded',\n");
    output.push_str(
        "          serviceDegradationReasonCode: 'missing-snapshot-rows',\n",
    );
    output.push_str("        };\n");
    output.push_str("  }\n\n");
    output.push_str("  if (connectionState === 'connecting') {\n");
    output.push_str("    return {\n");
    output.push_str("      serviceReachability: 'degraded',\n");
    output.push_str("      serviceDegradationReasonCode: 'connecting',\n");
    output.push_str("    };\n");
    output.push_str("  }\n\n");
    output.push_str("  if (hasSnapshotRows) {\n");
    output.push_str("    return {\n");
    output.push_str("      serviceReachability: 'degraded',\n");
    output.push_str("      serviceDegradationReasonCode: 'stale-snapshot-rows',\n");
    output.push_str("    };\n");
    output.push_str("  }\n\n");
    output.push_str("  return {\n");
    output.push_str("    serviceReachability: 'unavailable',\n");
    output.push_str("    serviceDegradationReasonCode: 'service-unavailable',\n");
    output.push_str("  };\n");
    output.push_str("}\n");
    output
}
