use ocentra_schema::parent_ui_bridge::{
    ParentSetupFirstRunPanelCardSnapshot, ParentSetupFirstRunPanelDetailSnapshot,
};

use super::{card, detail};

pub(super) fn first_run_state_card() -> ParentSetupFirstRunPanelCardSnapshot {
    let mut details = Vec::new();
    details.extend(first_run_account_details());
    details.extend(first_run_household_details());
    details.extend(first_run_device_details());
    details.extend(first_run_membership_details());
    details.extend(first_run_expiry_details());
    details.push(first_run_child_safety_detail());

    card(
        "First-run states and next actions",
        "The parent route names each setup boundary and the safe next action. Status stays unavailable until the owning authority supplies a typed, current read model.",
        details,
    )
}

fn first_run_account_details() -> Vec<ParentSetupFirstRunPanelDetailSnapshot> {
    vec![
        detail(
            "No account / session",
            "unavailable — Account/session owner must provide current state",
        ),
        detail(
            "Next action — no account / session",
            "manual-required — request an owner-backed current session",
        ),
        detail(
            "Account exists / no household",
            "unavailable — Account authority must provide household membership state",
        ),
        detail(
            "Next action — account exists / no household",
            "manual-required — request owner-backed household membership",
        ),
    ]
}

fn first_run_household_details() -> Vec<ParentSetupFirstRunPanelDetailSnapshot> {
    vec![
        detail(
            "Household exists / no child profile",
            "unavailable — family authority must provide child-profile state",
        ),
        detail(
            "Next action — household exists / no child profile",
            "manual-required — request an owner-backed child profile",
        ),
    ]
}

fn first_run_device_details() -> Vec<ParentSetupFirstRunPanelDetailSnapshot> {
    vec![
        detail(
            "Child profile exists / no device",
            "unavailable — setup/device-trust owner must provide device state",
        ),
        detail(
            "Next action — child profile exists / no device",
            "manual-required — request owner-backed device registration",
        ),
        detail(
            "Discovered unpaired device",
            "unavailable — LAN may only observe discovery; pairing and ownership are not bound",
        ),
        detail(
            "Next action — discovered unpaired device",
            "manual-required — use the trusted pairing owner flow",
        ),
        detail(
            "Paired device / service unavailable",
            "unavailable — child-service owner must provide current availability",
        ),
        detail(
            "Next action — paired device / service unavailable",
            "manual-required — wait for an owner-backed service receipt",
        ),
    ]
}

fn first_run_membership_details() -> Vec<ParentSetupFirstRunPanelDetailSnapshot> {
    vec![
        detail(
            "Parent controller / co-parent / observer",
            "unavailable — Account authority must provide the membership role and controller lease",
        ),
        detail(
            "Next action — parent role",
            "manual-required — request the owner-backed role and lease state",
        ),
    ]
}

fn first_run_expiry_details() -> Vec<ParentSetupFirstRunPanelDetailSnapshot> {
    vec![
        detail(
            "Invite expiry",
            "unavailable — invite owner must report active, expired, or consumed state",
        ),
        detail(
            "Next action — invite expiry",
            "manual-required — request a current invite receipt before retrying",
        ),
        detail(
            "Pairing expiry",
            "unavailable — trusted pairing owner must report active, expired, or revoked state",
        ),
        detail(
            "Next action — pairing expiry",
            "manual-required — request a current pairing receipt before retrying",
        ),
        detail(
            "Session expiry",
            "unavailable — Account/session owner must report fresh, stale, or expired state",
        ),
        detail(
            "Next action — session expiry",
            "manual-required — request a current session receipt before retrying",
        ),
        detail(
            "Recovery expiry",
            "unavailable — recovery owner must report current or expired state",
        ),
        detail(
            "Next action — recovery expiry",
            "manual-required — request a current recovery receipt before retrying",
        ),
    ]
}

fn first_run_child_safety_detail() -> ParentSetupFirstRunPanelDetailSnapshot {
    detail(
        "Child safety",
        "Private child activity is not shown on setup; only authority and readiness boundaries are projected",
    )
}
