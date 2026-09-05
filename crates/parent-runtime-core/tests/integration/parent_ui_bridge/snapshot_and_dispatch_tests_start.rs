use super::*;

#[test]
fn parent_route_snapshot_serializes_with_host_bridge_snapshot_fields() {
    let value = projected_route_snapshot_json(
        ParentRouteId::Activity,
        activity_route_projection(),
        TestContext("parent route snapshot serializes"),
    );

    assert_eq!(value["schemaVersion"], 1);
    assert_eq!(value["route"], "activity");
    assert_eq!(value["agentEndpoint"], "host-bridge://tauri-parent");
    assert_eq!(value["connectionState"], "connected");
    assert_eq!(value["seasonLabel"], "live");
    assert_eq!(value["dataSource"], "host-bridge");
    assert_eq!(value["summary"]["title"], "Activity");
    assert_eq!(value["summary"]["routeCapability"], "available");
    assert_eq!(value["summary"]["parentAccess"], "proof-missing");
    assert!(value["parentPortalShellStatus"].is_object());
    assert!(value["liveActivity"].is_object());
}

#[test]
fn start_route_snapshot_attaches_setup_first_run_panel() {
    let value = projected_route_snapshot_json(
        ParentRouteId::Start,
        lan_status_projection(sample_lan_read_model()),
        TestContext("start route snapshot serializes"),
    );

    assert_eq!(value["route"], "start");
    assert_eq!(
        value["setupFirstRunPanel"]["title"],
        "Setup-first-run boundary status"
    );
    assert_eq!(
        value["setupFirstRunPanel"]["summaryCardTitle"],
        "Current boundary status"
    );
    assert_eq!(
        value["setupFirstRunPanel"]["summaryDetails"][1]["value"],
        "unavailable"
    );
    assert_eq!(
        value["setupFirstRunPanel"]["cards"][1]["details"][0]["value"],
        "not wired"
    );
    assert_eq!(
        value["setupFirstRunPanel"]["cards"][3]["details"][1]["value"],
        "presentation only"
    );
}

#[test]
fn start_route_keeps_account_and_session_states_owner_gated() {
    let value = owner_gated_start_route_snapshot();
    let state_details = first_run_state_details(&value);

    assert_panel_detail_value(
        state_details,
        TestLabel("No account / session"),
        TestValue("unavailable — Account/session owner must provide current state"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — no account / session"),
        TestValue("manual-required — request an owner-backed current session"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Account exists / no household"),
        TestValue("unavailable — Account authority must provide household membership state"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — account exists / no household"),
        TestValue("manual-required — request owner-backed household membership"),
    );
}

#[test]
fn start_route_keeps_household_and_child_profile_states_owner_gated() {
    let value = owner_gated_start_route_snapshot();
    let missing_details = first_run_missing_details(&value);
    let state_details = first_run_state_details(&value);

    assert_panel_detail_value(
        missing_details,
        TestLabel("Child profile state"),
        TestValue("manual-required"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Household exists / no child profile"),
        TestValue("unavailable — family authority must provide child-profile state"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — household exists / no child profile"),
        TestValue("manual-required — request an owner-backed child profile"),
    );
}

#[test]
fn start_route_keeps_device_and_pairing_states_owner_gated() {
    let value = owner_gated_start_route_snapshot();
    let state_details = first_run_state_details(&value);

    assert_panel_detail_value(
        state_details,
        TestLabel("Child profile exists / no device"),
        TestValue("unavailable — setup/device-trust owner must provide device state"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — child profile exists / no device"),
        TestValue("manual-required — request owner-backed device registration"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Discovered unpaired device"),
        TestValue(
            "unavailable — LAN may only observe discovery; pairing and ownership are not bound",
        ),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — discovered unpaired device"),
        TestValue("manual-required — use the trusted pairing owner flow"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Paired device / service unavailable"),
        TestValue("unavailable — child-service owner must provide current availability"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — paired device / service unavailable"),
        TestValue("manual-required — wait for an owner-backed service receipt"),
    );
}

#[test]
fn start_route_keeps_membership_roles_owner_gated() {
    let value = owner_gated_start_route_snapshot();
    let missing_details = first_run_missing_details(&value);
    let state_details = first_run_state_details(&value);

    assert_panel_detail_value(
        missing_details,
        TestLabel("Parent controller role"),
        TestValue("manual-required — membership and controller lease owner not bound"),
    );
    assert_panel_detail_value(
        missing_details,
        TestLabel("Co-parent role"),
        TestValue("manual-required — membership role owner not bound"),
    );
    assert_panel_detail_value(
        missing_details,
        TestLabel("Observer role"),
        TestValue("manual-required — membership role owner not bound; observer remains read-only"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Parent controller / co-parent / observer"),
        TestValue(
            "unavailable — Account authority must provide the membership role and controller lease",
        ),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — parent role"),
        TestValue("manual-required — request the owner-backed role and lease state"),
    );
}

#[test]
fn start_route_keeps_expiry_states_owner_gated() {
    let value = owner_gated_start_route_snapshot();
    let state_details = first_run_state_details(&value);

    assert_panel_detail_value(
        state_details,
        TestLabel("Invite expiry"),
        TestValue("unavailable — invite owner must report active, expired, or consumed state"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — invite expiry"),
        TestValue("manual-required — request a current invite receipt before retrying"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Pairing expiry"),
        TestValue(
            "unavailable — trusted pairing owner must report active, expired, or revoked state",
        ),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — pairing expiry"),
        TestValue("manual-required — request a current pairing receipt before retrying"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Session expiry"),
        TestValue("unavailable — Account/session owner must report fresh, stale, or expired state"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — session expiry"),
        TestValue("manual-required — request a current session receipt before retrying"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Recovery expiry"),
        TestValue("unavailable — recovery owner must report current or expired state"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — recovery expiry"),
        TestValue("manual-required — request a current recovery receipt before retrying"),
    );
}

#[test]
fn start_route_keeps_child_activity_private_on_first_run() {
    let value = owner_gated_start_route_snapshot();
    let state_details = first_run_state_details(&value);

    assert_panel_detail_value(
        state_details,
        TestLabel("Child safety"),
        TestValue(
            "Private child activity is not shown on setup; only authority and readiness boundaries are projected",
        ),
    );
}

fn owner_gated_start_route_snapshot() -> Value {
    with_isolated_agent_addr(|| {
        route_snapshot_json(
            ParentRouteId::Start,
            None,
            TestContext("start route first-run states serialize with owner boundaries"),
        )
    })
}

fn first_run_missing_details(snapshot: &Value) -> &Value {
    &snapshot["setupFirstRunPanel"]["cards"][1]["details"]
}

fn first_run_state_details(snapshot: &Value) -> &Value {
    &snapshot["setupFirstRunPanel"]["cards"][2]["details"]
}
