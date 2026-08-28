use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel;
use ocentra_schema::parent_ui_bridge::{
    ParentRouteId, ParentSetupFirstRunPanelCardSnapshot, ParentSetupFirstRunPanelDetailSnapshot,
    ParentSetupFirstRunPanelSnapshot,
};
use serde::Serialize;

const SETUP_FIRST_RUN_PRODUCT_CLAIM: &str = "This panel reports only whether the Start route has a live Rust-owned setup-first-run snapshot. It does not claim live account readiness, signed installer readiness, pairing trust, data-custody execution, or onboarding completion.";
const MANUAL_REQUIRED: &str = "manual-required";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SetupFirstRunEvaluationState {
    ManualRequired,
}

impl SetupFirstRunEvaluationState {
    fn label(self) -> &'static str {
        match self {
            Self::ManualRequired => MANUAL_REQUIRED,
        }
    }
}

pub(crate) enum SetupFirstRunLanInput<'a> {
    NotRequested,
    Available(&'a LanBrowserAddDeviceReadModel),
    Unavailable {
        reason: &'static str,
        diagnostic_captured: bool,
    },
}

struct SetupFirstRunRuntimeSnapshot {
    evaluation_state: SetupFirstRunEvaluationState,
    account_state: SetupFirstRunBoundaryStatus,
    household_state: SetupFirstRunBoundaryStatus,
    session_state: SetupFirstRunBoundaryStatus,
    invite_state: SetupFirstRunBoundaryStatus,
    recovery_state: SetupFirstRunBoundaryStatus,
    device_authority_state: SetupFirstRunBoundaryStatus,
    lan_observation: SetupFirstRunLanObservation,
}

struct SetupFirstRunBoundaryStatus {
    state: &'static str,
    detail: &'static str,
}

struct SetupFirstRunLanObservation {
    source: &'static str,
    query_state: &'static str,
    selected_device: &'static str,
    selected_device_state: String,
    pairing_observation: String,
    reachability_observation: String,
    diagnostic_state: &'static str,
    canonical_device_count: usize,
    trusted_device_count: usize,
    revoked_device_count: usize,
}

const ACCOUNT_STATE: SetupFirstRunBoundaryStatus = SetupFirstRunBoundaryStatus {
    state: MANUAL_REQUIRED,
    detail: "no shipped provider-to-Account authority caller is bound to the parent route",
};
const HOUSEHOLD_STATE: SetupFirstRunBoundaryStatus = SetupFirstRunBoundaryStatus {
    state: MANUAL_REQUIRED,
    detail: "household membership must come from Account authority, not a route or LAN request",
};
const SESSION_STATE: SetupFirstRunBoundaryStatus = SetupFirstRunBoundaryStatus {
    state: MANUAL_REQUIRED,
    detail: "account/session lifecycle routes are not reachable from this parent snapshot",
};
const INVITE_STATE: SetupFirstRunBoundaryStatus = SetupFirstRunBoundaryStatus {
    state: MANUAL_REQUIRED,
    detail: "invite and observer/co-parent owner adapters are not bound to the parent route",
};
const RECOVERY_STATE: SetupFirstRunBoundaryStatus = SetupFirstRunBoundaryStatus {
    state: MANUAL_REQUIRED,
    detail: "recovery remains owner-receipt gated and cannot be inferred from UI state",
};

pub(crate) fn setup_first_run_panel_snapshot(
    route: &ParentRouteId,
    lan_input: SetupFirstRunLanInput<'_>,
) -> Option<ParentSetupFirstRunPanelSnapshot> {
    if !matches!(route, ParentRouteId::Start) {
        return None;
    }

    let runtime = load_runtime_snapshot(lan_input);
    Some(ParentSetupFirstRunPanelSnapshot {
        eyebrow: "Setup route".to_string(),
        title: "Setup-first-run boundary status".to_string(),
        body: "The Start route exists, but live setup-first-run runtime state is not yet wired into the Rust parent snapshot. This panel reports that gap honestly instead of inventing onboarding progress.".to_string(),
        summary_card_title: "Current boundary status".to_string(),
        summary: "Portal rendering and the Rust-owned route snapshot exist, but live setup/account/trust/custody state is unavailable here today.".to_string(),
        summary_details: summary_details(&runtime),
        cards: vec![
            what_is_real_now_card(&runtime),
            what_is_missing_card(&runtime),
            first_run_state_card(),
            where_it_belongs_card(&runtime),
        ],
        product_claim: SETUP_FIRST_RUN_PRODUCT_CLAIM.to_string(),
    })
}

fn load_runtime_snapshot(lan_input: SetupFirstRunLanInput<'_>) -> SetupFirstRunRuntimeSnapshot {
    SetupFirstRunRuntimeSnapshot {
        evaluation_state: SetupFirstRunEvaluationState::ManualRequired,
        account_state: ACCOUNT_STATE,
        household_state: HOUSEHOLD_STATE,
        session_state: SESSION_STATE,
        invite_state: INVITE_STATE,
        recovery_state: RECOVERY_STATE,
        device_authority_state: SetupFirstRunBoundaryStatus {
            state: MANUAL_REQUIRED,
            detail: "trusted device authority is not supplied by the LAN read model",
        },
        lan_observation: lan_observation(lan_input),
    }
}

fn lan_observation(lan_input: SetupFirstRunLanInput<'_>) -> SetupFirstRunLanObservation {
    match lan_input {
        SetupFirstRunLanInput::NotRequested => SetupFirstRunLanObservation {
            source: "unavailable",
            query_state: "not-requested",
            selected_device: "unavailable",
            selected_device_state: "unavailable".to_string(),
            pairing_observation: "unavailable".to_string(),
            reachability_observation: "unavailable".to_string(),
            diagnostic_state: "not-applicable",
            canonical_device_count: 0,
            trusted_device_count: 0,
            revoked_device_count: 0,
        },
        SetupFirstRunLanInput::Available(read_model) => {
            let selected_device = read_model
                .selected_device_readiness
                .selected_child_device_id
                .is_some();
            let selected_device_state = selected_device_status(read_model);
            SetupFirstRunLanObservation {
                source: "LAN",
                query_state: "available",
                selected_device: selected_device_label(selected_device),
                selected_device_state,
                pairing_observation: serialized_label(
                    &read_model.selected_device_readiness.trust_state,
                ),
                reachability_observation: serialized_label(
                    &read_model.selected_device_readiness.reachability,
                ),
                diagnostic_state: "not-applicable",
                canonical_device_count: read_model.canonical_household_devices.len(),
                trusted_device_count: read_model.trusted_device_registry.len(),
                revoked_device_count: read_model.revoked_device_ids.len(),
            }
        }
        SetupFirstRunLanInput::Unavailable {
            reason,
            diagnostic_captured,
        } => SetupFirstRunLanObservation {
            source: "unavailable",
            query_state: reason,
            selected_device: "unavailable",
            selected_device_state: "unavailable".to_string(),
            pairing_observation: "unavailable".to_string(),
            reachability_observation: "unavailable".to_string(),
            diagnostic_state: if diagnostic_captured {
                "captured-in-rust-bridge"
            } else {
                "no-diagnostic-detail"
            },
            canonical_device_count: 0,
            trusted_device_count: 0,
            revoked_device_count: 0,
        },
    }
}

fn selected_device_label(selected_device: bool) -> &'static str {
    ["not-selected", "selected"][usize::from(selected_device)]
}

fn selected_device_status(read_model: &LanBrowserAddDeviceReadModel) -> String {
    format!(
        "observation; trust={}, reachability={}, control={}, authority=unavailable",
        serialized_label(&read_model.selected_device_readiness.trust_state),
        serialized_label(&read_model.selected_device_readiness.reachability),
        read_model.selected_device_readiness.ready_for_control,
    )
}

fn summary_details(
    _runtime: &SetupFirstRunRuntimeSnapshot,
) -> Vec<ParentSetupFirstRunPanelDetailSnapshot> {
    vec![
        detail("Route", "start"),
        detail("Runtime state", "unavailable"),
        detail("Snapshot owner", "Rust parent runtime host bridge"),
        detail("Product claim", SETUP_FIRST_RUN_PRODUCT_CLAIM),
    ]
}

fn what_is_real_now_card(
    runtime: &SetupFirstRunRuntimeSnapshot,
) -> ParentSetupFirstRunPanelCardSnapshot {
    card(
        "What is real now",
        "The Start route can render an honest Rust-owned boundary panel without inventing setup progress.",
        vec![
            detail("Route shell", "Start route is visible in the portal shell"),
            detail("Snapshot transport", "Host bridge snapshot reaches TS presentation"),
            detail("Evidence boundary", "Route-contract projection only"),
            detail("LAN source", runtime.lan_observation.source),
            detail("LAN query state", runtime.lan_observation.query_state),
            detail("Selected device", runtime.lan_observation.selected_device),
            detail("Selected device status", &runtime.lan_observation.selected_device_state),
            detail("Pairing observation", &runtime.lan_observation.pairing_observation),
            detail(
                "Reachability observation",
                &runtime.lan_observation.reachability_observation,
            ),
            detail(
                "LAN authority",
                "observation only; ownership and trust remain unavailable",
            ),
            detail(
                "Canonical household rows",
                &runtime.lan_observation.canonical_device_count.to_string(),
            ),
            detail(
                "Trusted registry rows",
                &runtime.lan_observation.trusted_device_count.to_string(),
            ),
            detail(
                "Revoked device rows",
                &runtime.lan_observation.revoked_device_count.to_string(),
            ),
            detail("Diagnostic detail", runtime.lan_observation.diagnostic_state),
        ],
    )
}

fn what_is_missing_card(
    runtime: &SetupFirstRunRuntimeSnapshot,
) -> ParentSetupFirstRunPanelCardSnapshot {
    card(
        "What is missing",
        "No live setup-first-run read model is wired here yet, so the panel must stay explicit about the missing runtime state.",
        vec![
            detail("Account/provider state", "not wired"),
            detail("Pairing/trust state", "not wired"),
            detail("Data-custody/readiness state", "not wired"),
            detail("Completion claim", "withheld until a live Rust snapshot exists"),
            detail("Setup state", runtime.evaluation_state.label()),
            detail("Account identity", runtime.account_state.state),
            detail("Account boundary", runtime.account_state.detail),
            detail("Household membership", runtime.household_state.state),
            detail("Household boundary", runtime.household_state.detail),
            detail("Session lifecycle", runtime.session_state.state),
            detail("Session boundary", runtime.session_state.detail),
            detail("Invite / observer access", runtime.invite_state.state),
            detail("Invite boundary", runtime.invite_state.detail),
            detail("Recovery", runtime.recovery_state.state),
            detail("Recovery boundary", runtime.recovery_state.detail),
            detail("Device authority", runtime.device_authority_state.state),
            detail("Device authority boundary", runtime.device_authority_state.detail),
            detail("Child profile state", MANUAL_REQUIRED),
            detail(
                "Parent controller role",
                "manual-required — membership and controller lease owner not bound",
            ),
            detail(
                "Co-parent role",
                "manual-required — membership role owner not bound",
            ),
            detail(
                "Observer role",
                "manual-required — membership role owner not bound; observer remains read-only",
            ),
        ],
    )
}

fn first_run_state_card() -> ParentSetupFirstRunPanelCardSnapshot {
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

fn where_it_belongs_card(
    runtime: &SetupFirstRunRuntimeSnapshot,
) -> ParentSetupFirstRunPanelCardSnapshot {
    card(
        "Where it belongs",
        "When first-run becomes live, Rust must own the setup state and TS must remain pure rendering.",
        vec![
            detail("Rust owner", "parent runtime + setup read model"),
            detail("TS role", "presentation only"),
            detail("Proof rule", "claim only what the live Rust snapshot can prove"),
            detail("Source and custody", "Rust-owned boundary; unavailable owners stay explicit"),
            detail("Live local", "unavailable"),
            detail("LAN", runtime.lan_observation.source),
            detail("Parent cache", "unavailable"),
            detail("Parent-owned storage", "unavailable — data-custody owner not bound"),
            detail("Cloud relay", "unavailable"),
            detail("Child activity custody", "unavailable — data-custody owner"),
            detail("Degraded/manual state", MANUAL_REQUIRED),
            detail("Provisioning evaluator", "not invoked"),
            detail("Account mutation", "not invoked"),
            detail("Action planning", "not invoked"),
        ],
    )
}

fn card(
    title: &str,
    summary: &str,
    details: Vec<ParentSetupFirstRunPanelDetailSnapshot>,
) -> ParentSetupFirstRunPanelCardSnapshot {
    ParentSetupFirstRunPanelCardSnapshot {
        title: title.to_string(),
        summary: summary.to_string(),
        details,
    }
}

fn detail(label: &str, value: &str) -> ParentSetupFirstRunPanelDetailSnapshot {
    ParentSetupFirstRunPanelDetailSnapshot {
        label: label.to_string(),
        value: value.to_string(),
    }
}

fn serialized_label<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value)
        .unwrap_or_else(|_| "unavailable".to_string())
        .trim_matches('"')
        .to_string()
}
