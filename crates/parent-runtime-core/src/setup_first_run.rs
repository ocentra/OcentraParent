use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel;
use ocentra_schema::parent_ui_bridge::{
    ParentRouteId, ParentSetupFirstRunPanelCardSnapshot, ParentSetupFirstRunPanelDetailSnapshot,
    ParentSetupFirstRunPanelSnapshot,
};
use serde::Serialize;

const SETUP_FIRST_RUN_PRODUCT_CLAIM: &str = "This Start-route panel reports the availability of trusted first-run inputs. It does not run the provisioning evaluator, infer household ownership or device trust from LAN selection, or claim onboarding completion while required authorities are unavailable.";
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
    lan_observation: SetupFirstRunLanObservation,
}

struct SetupFirstRunLanObservation {
    source: &'static str,
    query_state: &'static str,
    selected_device: &'static str,
    pairing_observation: String,
    reachability_observation: String,
    diagnostic_state: &'static str,
}

#[derive(Clone, Copy)]
struct SetupFirstRunAuthorityRequirement {
    label: &'static str,
    owner: &'static str,
}

const REQUIRED_AUTHORITIES: [SetupFirstRunAuthorityRequirement; 15] = [
    authority("Household membership", "account identity authority"),
    authority(
        "Account readiness",
        "account session and recovery authority",
    ),
    authority(
        "Parent app readiness",
        "signed parent package/runtime authority",
    ),
    authority(
        "Parent device registration",
        "device registration authority",
    ),
    authority(
        "Child install state",
        "signed child package installer authority",
    ),
    authority("Child service state", "child runtime service authority"),
    authority("Child app readiness", "child runtime readiness authority"),
    authority(
        "Child device ownership",
        "household device-binding authority",
    ),
    authority("Device trust", "device-trust authority"),
    authority("Permission readiness", "platform permission authority"),
    authority("Pairing lifecycle", "trusted pairing authority"),
    authority("Policy baseline", "policy control-plane authority"),
    authority("Data custody sync", "storage custody authority"),
    authority(
        "Network reachability",
        "trusted network readiness authority",
    ),
    authority("Recovery state", "account/device recovery authority"),
];

const fn authority(label: &'static str, owner: &'static str) -> SetupFirstRunAuthorityRequirement {
    SetupFirstRunAuthorityRequirement { label, owner }
}

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
        title: "First-run authority status".to_string(),
        body: "Rust exposes the complete first-run input matrix, but trusted owner inputs are not yet bound. Evaluation is withheld and the route remains manual-required.".to_string(),
        summary_card_title: "Current boundary status".to_string(),
        summary: format!(
            "Provisioning evaluation was not run: {} of {} trusted readiness inputs are unavailable; setup is {}.",
            REQUIRED_AUTHORITIES.len(),
            REQUIRED_AUTHORITIES.len(),
            runtime.evaluation_state.label(),
        ),
        summary_details: summary_details(&runtime),
        cards: vec![
            readiness_matrix_card(),
            lan_observation_card(&runtime.lan_observation),
            execution_boundary_card(),
        ],
        product_claim: SETUP_FIRST_RUN_PRODUCT_CLAIM.to_string(),
    })
}

fn load_runtime_snapshot(lan_input: SetupFirstRunLanInput<'_>) -> SetupFirstRunRuntimeSnapshot {
    SetupFirstRunRuntimeSnapshot {
        evaluation_state: SetupFirstRunEvaluationState::ManualRequired,
        lan_observation: lan_observation(lan_input),
    }
}

fn lan_observation(lan_input: SetupFirstRunLanInput<'_>) -> SetupFirstRunLanObservation {
    match lan_input {
        SetupFirstRunLanInput::NotRequested => SetupFirstRunLanObservation {
            source: "not-requested",
            query_state: "not-requested",
            selected_device: "unavailable",
            pairing_observation: "unavailable".to_string(),
            reachability_observation: "unavailable".to_string(),
            diagnostic_state: "not-applicable",
        },
        SetupFirstRunLanInput::Available(read_model) => SetupFirstRunLanObservation {
            source: "live-lan-read-model-observation-only",
            query_state: "available",
            selected_device: if read_model
                .selected_device_readiness
                .selected_child_device_id
                .is_some()
            {
                "selected"
            } else {
                "not-selected"
            },
            pairing_observation: serialized_label(
                &read_model.selected_device_readiness.trust_state,
            ),
            reachability_observation: serialized_label(
                &read_model.selected_device_readiness.reachability,
            ),
            diagnostic_state: "not-applicable",
        },
        SetupFirstRunLanInput::Unavailable {
            reason,
            diagnostic_captured,
        } => SetupFirstRunLanObservation {
            source: "lan-read-model-unavailable",
            query_state: reason,
            selected_device: "unavailable",
            pairing_observation: "unavailable".to_string(),
            reachability_observation: "unavailable".to_string(),
            diagnostic_state: if diagnostic_captured {
                "captured-in-rust-bridge"
            } else {
                "no-diagnostic-detail"
            },
        },
    }
}

fn summary_details(
    runtime: &SetupFirstRunRuntimeSnapshot,
) -> Vec<ParentSetupFirstRunPanelDetailSnapshot> {
    vec![
        detail("Route", "start"),
        detail("Evaluation state", "not-run"),
        detail("Setup state", runtime.evaluation_state.label()),
        detail("Trusted inputs available", "0"),
        detail(
            "Trusted inputs unavailable",
            &REQUIRED_AUTHORITIES.len().to_string(),
        ),
        detail("LAN query state", runtime.lan_observation.query_state),
        detail("Snapshot owner", "Rust parent runtime host bridge"),
        detail("Product claim", SETUP_FIRST_RUN_PRODUCT_CLAIM),
    ]
}

fn readiness_matrix_card() -> ParentSetupFirstRunPanelCardSnapshot {
    card(
        "Trusted readiness input matrix",
        "Every provisioning input remains unavailable until its owning runtime supplies typed, authenticated state. No fallback enum value is substituted.",
        REQUIRED_AUTHORITIES
            .iter()
            .map(|requirement| {
                detail(
                    requirement.label,
                    &format!("{MANUAL_REQUIRED} — {} unavailable", requirement.owner),
                )
            })
            .collect(),
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
            detail(
                "Reachability observation",
                &observation.reachability_observation,
            ),
            detail("Diagnostic detail", observation.diagnostic_state),
            detail(
                "Authority boundary",
                "observation only; ownership and trust remain unavailable",
            ),
        ],
    )
}

fn execution_boundary_card() -> ParentSetupFirstRunPanelCardSnapshot {
    card(
        "Execution boundary",
        "The shipped Start route reaches this Rust snapshot through the host bridge. Provisioning can advance only after all owner authorities provide trusted inputs.",
        vec![
            detail(
                "Reachable path",
                "parent_load_route -> parent runtime -> setup-first-run panel",
            ),
            detail("Current effect", "render manual-required authority matrix"),
            detail("Provisioning evaluator", "not invoked"),
            detail("Action planning", "not invoked"),
            detail("TS ownership", "presentation only"),
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
