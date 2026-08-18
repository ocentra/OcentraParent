use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel;
use ocentra_schema::parent_ui_bridge::{
    ParentRouteId, ParentSetupFirstRunPanelCardSnapshot, ParentSetupFirstRunPanelDetailSnapshot,
    ParentSetupFirstRunPanelSnapshot,
};
use serde::Serialize;

const SETUP_FIRST_RUN_PRODUCT_CLAIM: &str = "This Start-route panel projects only Rust-owned setup inputs. Account, household, session, invite, recovery, and trusted-device actions stay manual-required until their owning runtime is reachable.";
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
        title: "First-run account and family status".to_string(),
        body: "The Start route is reachable, but setup actions are withheld until Account, session, household, invite/recovery, and trusted-device owners provide typed runtime state.".to_string(),
        summary_card_title: "Current setup boundary".to_string(),
        summary: format!(
            "First-run evaluation is {}. Account and family mutation remains unavailable; the panel reports the reachable LAN observation without promoting it to authority.",
            runtime.evaluation_state.label(),
        ),
        summary_details: summary_details(&runtime),
        cards: vec![
            first_run_state_machine_card(&runtime),
            account_family_status_card(&runtime),
            device_authority_card(&runtime),
            invite_recovery_card(&runtime),
            source_custody_card(&runtime),
            lan_observation_card(&runtime.lan_observation),
            execution_boundary_card(&runtime),
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
    runtime: &SetupFirstRunRuntimeSnapshot,
) -> Vec<ParentSetupFirstRunPanelDetailSnapshot> {
    vec![
        detail("Route", "start"),
        detail("First-run state", runtime.evaluation_state.label()),
        detail("Account identity", runtime.account_state.state),
        detail("Household membership", runtime.household_state.state),
        detail("Session lifecycle", runtime.session_state.state),
        detail("Device authority", runtime.device_authority_state.state),
        detail("LAN source", runtime.lan_observation.source),
        detail("Snapshot owner", "Rust parent runtime host bridge"),
    ]
}

fn first_run_state_machine_card(
    runtime: &SetupFirstRunRuntimeSnapshot,
) -> ParentSetupFirstRunPanelCardSnapshot {
    card(
        "First-run state machine",
        "No state transition is inferred from login, route selection, or LAN discovery. Each next step remains blocked until its owner supplies a typed state.",
        vec![
            detail("Welcome / sign-in", runtime.account_state.state),
            detail("Signed-in, no household", runtime.household_state.state),
            detail("Create or join household", runtime.household_state.state),
            detail("Create child profile", runtime.household_state.state),
            detail("Add or pair child device", runtime.device_authority_state.state),
            detail("Invite co-parent / observer", runtime.invite_state.state),
            detail("Recovery", runtime.recovery_state.state),
            detail("Next transition", "manual-required — no trusted owner input"),
        ],
    )
}

fn account_family_status_card(
    runtime: &SetupFirstRunRuntimeSnapshot,
) -> ParentSetupFirstRunPanelCardSnapshot {
    card(
        "Account and household authority",
        "The parent UI may display these boundaries, but it cannot create membership, roles, profiles, or sessions without the Account authority caller.",
        vec![
            detail("Account identity", runtime.account_state.state),
            detail("Account boundary", runtime.account_state.detail),
            detail("Household membership", runtime.household_state.state),
            detail("Household boundary", runtime.household_state.detail),
            detail("Session lifecycle", runtime.session_state.state),
            detail("Session boundary", runtime.session_state.detail),
            detail("Child profile", "manual-required — no account-owned profile read model"),
            detail("Role visibility", "manual-required — no membership role read model"),
        ],
    )
}

fn device_authority_card(
    runtime: &SetupFirstRunRuntimeSnapshot,
) -> ParentSetupFirstRunPanelCardSnapshot {
    card(
        "Child-device authority",
        "LAN may report discovery, pairing, and reachability observations. It does not mint household membership, device ownership, trust, controller lease, or child capability authority.",
        vec![
            detail("Device authority", runtime.device_authority_state.state),
            detail("Authority boundary", runtime.device_authority_state.detail),
            detail("Selected device", runtime.lan_observation.selected_device),
            detail("Selected device status", &runtime.lan_observation.selected_device_state),
            detail("Canonical household rows", &runtime.lan_observation.canonical_device_count.to_string()),
            detail("Trusted registry rows", &runtime.lan_observation.trusted_device_count.to_string()),
            detail("Revoked device rows", &runtime.lan_observation.revoked_device_count.to_string()),
            detail("Physical pairing proof", "manual-required — device-trust and LAN owners"),
        ],
    )
}

fn invite_recovery_card(
    runtime: &SetupFirstRunRuntimeSnapshot,
) -> ParentSetupFirstRunPanelCardSnapshot {
    card(
        "Invites, observer access, and recovery",
        "Invite, support, and recovery states remain visible as blocked boundaries until identity, membership, rate/replay custody, and owner receipts are composed.",
        vec![
            detail("Co-parent invite", runtime.invite_state.state),
            detail("Observer invite", runtime.invite_state.state),
            detail("Observer write authority", "unavailable — observer remains read-only"),
            detail("Recovery state", runtime.recovery_state.state),
            detail("Recovery boundary", runtime.recovery_state.detail),
            detail("Support access", "manual-required — separate audited support owner"),
        ],
    )
}

fn source_custody_card(
    runtime: &SetupFirstRunRuntimeSnapshot,
) -> ParentSetupFirstRunPanelCardSnapshot {
    card(
        "Source and custody",
        "The panel labels where setup status came from and keeps unavailable custody explicit. It does not present hosted child activity or parent cache as available.",
        vec![
            detail("Live local", "unavailable"),
            detail("LAN", runtime.lan_observation.source),
            detail("Parent cache", "unavailable"),
            detail("Parent-owned storage", "unavailable — Account custody caller not bound"),
            detail("Cloud relay", "unavailable"),
            detail("Child activity custody", "unavailable — data-custody owner"),
            detail("Degraded/manual state", MANUAL_REQUIRED),
        ],
    )
}

fn lan_observation_card(
    observation: &SetupFirstRunLanObservation,
) -> ParentSetupFirstRunPanelCardSnapshot {
    card(
        "LAN observation (non-authoritative)",
        "LAN discovery may describe a selected endpoint, pairing observation, and reachability. It never establishes household ownership, device trust, pairing authority, or setup readiness.",
        vec![
            detail("Source", observation.source),
            detail("Query state", observation.query_state),
            detail("Selected endpoint", observation.selected_device),
            detail("Pairing observation", &observation.pairing_observation),
            detail("Reachability observation", &observation.reachability_observation),
            detail("Diagnostic detail", observation.diagnostic_state),
            detail("Authority boundary", "observation only; ownership and trust remain unavailable"),
        ],
    )
}

fn execution_boundary_card(
    _runtime: &SetupFirstRunRuntimeSnapshot,
) -> ParentSetupFirstRunPanelCardSnapshot {
    card(
        "Execution boundary",
        "The shipped Start route reaches this Rust snapshot through the host bridge. No setup mutation or provisioning evaluation is dispatched from this panel.",
        vec![
            detail("Reachable path", "parent_load_route -> parent runtime -> setup-first-run panel"),
            detail("Current effect", "render account/family status and manual-required boundaries"),
            detail("Provisioning evaluator", "not invoked"),
            detail("Account mutation", "not invoked"),
            detail("Action planning", "not invoked"),
            detail("TS ownership", "presentation only"),
            detail("Product claim", SETUP_FIRST_RUN_PRODUCT_CLAIM),
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
