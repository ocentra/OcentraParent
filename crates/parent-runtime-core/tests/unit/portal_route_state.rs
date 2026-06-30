use ocentra_parent_runtime_core::portal_route_state::{
    parent_portal_manage_lane_for_route, parent_portal_route_state, portal_route_from_hash_path,
    portal_route_state_typescript, resolve_parent_portal_service_reachability,
    ParentPortalManageLane, ParentPortalPageMode, ParentPortalServiceDegradationReasonCode,
    ParentPortalServiceReachability,
};

#[test]
fn portal_route_state_parses_hash_paths_only_for_known_routes() {
    assert_eq!(
        portal_route_from_hash_path("#/browser?panel=inventory"),
        Some("browser")
    );
    assert_eq!(
        portal_route_from_hash_path("#/notification-channels?panel=alerts"),
        Some("notification-channels")
    );
    assert_eq!(portal_route_from_hash_path("#/subscription"), Some("subscription"));
    assert_eq!(portal_route_from_hash_path("#/unknown"), None);
}

#[test]
fn portal_route_state_keeps_manage_lane_ownership_in_rust() {
    assert_eq!(
        parent_portal_manage_lane_for_route("browser"),
        Some(ParentPortalManageLane::ChildPolicy)
    );
    assert_eq!(
        parent_portal_manage_lane_for_route("notifications"),
        Some(ParentPortalManageLane::Portal)
    );
    assert_eq!(
        parent_portal_manage_lane_for_route("devices"),
        Some(ParentPortalManageLane::DeviceOps)
    );
    assert_eq!(parent_portal_manage_lane_for_route("overview"), None);
}

#[test]
fn portal_route_state_returns_selected_control_and_page_mode() {
    let browser = parent_portal_route_state("browser").expect("browser route");
    assert_eq!(browser.page_mode, ParentPortalPageMode::ParentManage);
    assert_eq!(browser.selected_control_id, "managed-web");
    assert_eq!(browser.manage_lane, Some(ParentPortalManageLane::ChildPolicy));

    let subscription = parent_portal_route_state("subscription").expect("subscription route");
    assert_eq!(subscription.page_mode, ParentPortalPageMode::ParentManage);
    assert_eq!(subscription.selected_control_id, "subscription-plans");
    assert_eq!(subscription.manage_lane, Some(ParentPortalManageLane::Portal));

    let policy = parent_portal_route_state("policy").expect("policy route");
    assert_eq!(policy.page_mode, ParentPortalPageMode::ParentGuide);
    assert_eq!(policy.selected_control_id, "rules-policy");
    assert_eq!(policy.manage_lane, None);
}

#[test]
fn portal_route_state_resolves_service_reachability_in_rust() {
    let connected = resolve_parent_portal_service_reachability("connected", true);
    assert_eq!(
        connected.service_reachability,
        ParentPortalServiceReachability::Reachable
    );
    assert_eq!(connected.service_degradation_reason_code, None);

    let missing_rows = resolve_parent_portal_service_reachability("connected", false);
    assert_eq!(
        missing_rows.service_reachability,
        ParentPortalServiceReachability::Degraded
    );
    assert_eq!(
        missing_rows.service_degradation_reason_code,
        Some(ParentPortalServiceDegradationReasonCode::MissingSnapshotRows)
    );

    let stale_rows = resolve_parent_portal_service_reachability("disconnected", true);
    assert_eq!(
        stale_rows.service_degradation_reason_code,
        Some(ParentPortalServiceDegradationReasonCode::StaleSnapshotRows)
    );

    let unavailable = resolve_parent_portal_service_reachability("error", false);
    assert_eq!(
        unavailable.service_reachability,
        ParentPortalServiceReachability::Unavailable
    );
    assert_eq!(
        unavailable.service_degradation_reason_code,
        Some(ParentPortalServiceDegradationReasonCode::ServiceUnavailable)
    );
}

#[test]
fn portal_route_state_generated_typescript_stays_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/portal-domain/src/generated/portal-route-state.ts"
    );

    assert_eq!(checked_in, portal_route_state_typescript());
}
