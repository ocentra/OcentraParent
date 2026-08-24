use ocentra_parent_agent_protocol::AppGamePlatformProofStatusReadModel;
use ocentra_parent_agent_protocol::AppGamePlatformProofStatusRow;
use ocentra_parent_agent_protocol::app_game::APP_GAME_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::app_game_adapter_execution_readiness::{
    APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE, APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_APPLICABLE,
    APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED, APP_GAME_ADAPTER_PRODUCT_NATIVE_APP,
    APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME, APP_GAME_PARENT_PLATFORM_IOS,
    APP_GAME_PARENT_PLATFORM_MACOS,
};
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    APP_GAME_PARENT_PLATFORM_ANDROID, APP_GAME_PARENT_PLATFORM_LINUX,
    APP_GAME_PARENT_PLATFORM_WINDOWS,
};
use ocentra_parent_agent_protocol::app_game_platform_proof_status::AppGameLinuxDockerHostPreflight;
use ocentra_parent_agent_protocol::app_game_platform_proof_status::{
    APP_GAME_LINUX_DOCKER_PREFLIGHT_READY, APP_GAME_PLATFORM_AUTHORITY_NOT_LOCALLY_PROVABLE,
    APP_GAME_PLATFORM_AUTHORITY_SCOPED_EXECUTION_ONLY, APP_GAME_PLATFORM_AUTHORITY_VISIBILITY_ONLY,
    APP_GAME_PLATFORM_GAP_ANDROID_DEVICE_OWNER, APP_GAME_PLATFORM_GAP_ANDROID_DURABLE_USAGE_REPLAY,
    APP_GAME_PLATFORM_GAP_BROAD_BLOCKING, APP_GAME_PLATFORM_GAP_CHILD_DELIVERY,
    APP_GAME_PLATFORM_GAP_IOS_ARTIFACTS, APP_GAME_PLATFORM_GAP_LINUX_DOCKER_PREFLIGHT,
    APP_GAME_PLATFORM_GAP_LINUX_FOREGROUND_CAPTURE, APP_GAME_PLATFORM_GAP_LINUX_NATIVE_SERVICE,
    APP_GAME_PLATFORM_GAP_LINUX_ROLLBACK, APP_GAME_PLATFORM_GAP_MACOS_ARTIFACTS,
    APP_GAME_PLATFORM_GAP_PLATFORM_ENFORCEMENT, APP_GAME_PLATFORM_PROOF_ANDROID_HOST_NOT_DETECTED,
    APP_GAME_PLATFORM_PROOF_ANDROID_HOST_VISIBLE, APP_GAME_PLATFORM_PROOF_LINUX_HOST_NOT_DETECTED,
    APP_GAME_PLATFORM_PROOF_LINUX_HOST_VISIBLE,
    APP_GAME_PLATFORM_PROOF_LOCAL_RUNTIME_NOT_APPLICABLE,
    APP_GAME_PLATFORM_PROOF_SCOPED_WINDOWS_EXECUTION,
    APP_GAME_PLATFORM_PROOF_STATUS_CAPABILITY_PARTIAL,
    APP_GAME_PLATFORM_PROOF_STATUS_CUSTODY_LABEL, APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL_ID,
    APP_GAME_PLATFORM_PROOF_STATUS_ROW_ID_PREFIX,
};
use ocentra_parent_agent_protocol::constants::{
    self, v08_supported_adapter_runtime_proof as proof,
};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};
use ocentra_parent_screen_capture_adapter::linux_foreground_source::LinuxForegroundSourcePreflight;

use super::app_game_adapter_execution_readiness_payload::GeneratedAtText;
use super::app_game_adapter_host_capabilities::HostCapabilitySignals;
use super::app_game_linux_docker_host_preflight::{
    detect_linux_docker_host_preflight, unavailable_linux_docker_host_preflight,
};
use crate::{event_builder::build_event, fields::fields_from_pairs, time::timestamp_now};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const PLATFORM_PROBE_MIN_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct TextValue(pub(super) &'static str);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct TextList(pub(super) Vec<String>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SerializedReadModelText(pub(super) String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PlatformProbeRequestProvenance {
    Loopback,
    LocalNetwork,
    Unknown,
}

#[derive(Clone)]
pub(crate) struct PlatformProbeCache {
    state: Arc<Mutex<PlatformProbeCacheState>>,
}

#[derive(Clone)]
struct PlatformProbeCacheState {
    snapshot: (HostCapabilitySignals, AppGameLinuxDockerHostPreflight),
    last_refresh: Option<Instant>,
    refresh_in_progress: bool,
}

impl PlatformProbeCache {
    pub(crate) fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(PlatformProbeCacheState {
                snapshot: unavailable_platform_host_state(),
                last_refresh: None,
                refresh_in_progress: false,
            })),
        }
    }

    fn snapshot_for(
        &self,
        provenance: PlatformProbeRequestProvenance,
    ) -> (HostCapabilitySignals, AppGameLinuxDockerHostPreflight) {
        let Ok(mut state) = self.state.lock() else {
            return unavailable_platform_host_state();
        };
        let refresh_allowed = provenance == PlatformProbeRequestProvenance::Loopback;
        let within_rate_limit = state
            .last_refresh
            .is_some_and(|last| last.elapsed() < PLATFORM_PROBE_MIN_INTERVAL);
        if !refresh_allowed || within_rate_limit || state.refresh_in_progress {
            return state.snapshot.clone();
        }
        state.refresh_in_progress = true;
        drop(state);

        let snapshot = detect_platform_host_state();
        let Ok(mut state) = self.state.lock() else {
            return unavailable_platform_host_state();
        };
        state.snapshot = snapshot.clone();
        state.last_refresh = Some(Instant::now());
        state.refresh_in_progress = false;
        snapshot
    }
}

pub async fn build_activity_app_game_platform_proof_status_report(
    command: AgentCommandEnvelope,
    probe_cache: PlatformProbeCache,
    provenance: PlatformProbeRequestProvenance,
) -> AgentEventEnvelope {
    let generated_at = GeneratedAtText(timestamp_now());
    let (host_capabilities, linux_docker_host_preflight) =
        match tokio::task::spawn_blocking(move || probe_cache.snapshot_for(provenance)).await {
            Ok(state) => state,
            Err(_) => unavailable_platform_host_state(),
        };
    // The live Linux foreground tool path is intentionally not spawned from a
    // request. No retained subprocess owner can guarantee custody across
    // setsid/pid-namespace escapes, so this production handler stays
    // unavailable until an owned single-flight worker with a real OS custody
    // primitive exists. Docker host visibility is an independent bounded
    // preflight and does not mint foreground-source authority.
    let linux_preflight = LinuxForegroundSourcePreflight::unavailable();
    let read_model = app_game_platform_proof_status_read_model_from_preflights(
        generated_at,
        &host_capabilities,
        &linux_preflight,
        &linux_docker_host_preflight,
    );
    build_event(
        constants::event_id::ACTIVITY_APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityAppGamePlatformProofStatusReadModelReported,
        LogLevel::Info,
        app_game_platform_proof_status_payload(&read_model),
        None,
    )
}

pub fn app_game_platform_proof_status_read_model(
    generated_at: GeneratedAtText,
) -> AppGamePlatformProofStatusReadModel {
    let (host_capabilities, linux_docker_host_preflight) = detect_platform_host_state();
    let linux_preflight = LinuxForegroundSourcePreflight::unavailable();
    app_game_platform_proof_status_read_model_from_preflights(
        generated_at,
        &host_capabilities,
        &linux_preflight,
        &linux_docker_host_preflight,
    )
}

pub fn app_game_platform_proof_status_read_model_with_linux_preflight(
    generated_at: GeneratedAtText,
    linux_preflight: LinuxForegroundSourcePreflight,
) -> AppGamePlatformProofStatusReadModel {
    let host_capabilities = HostCapabilitySignals::detect();
    app_game_platform_proof_status_read_model_from_preflights(
        generated_at,
        &host_capabilities,
        &linux_preflight,
        &unavailable_linux_docker_host_preflight(),
    )
}

fn app_game_platform_proof_status_read_model_from_preflights(
    generated_at: GeneratedAtText,
    host_capabilities: &HostCapabilitySignals,
    linux_preflight: &LinuxForegroundSourcePreflight,
    linux_docker_host_preflight: &AppGameLinuxDockerHostPreflight,
) -> AppGamePlatformProofStatusReadModel {
    let rows = platform_status_rows(
        &generated_at,
        host_capabilities,
        linux_preflight,
        linux_docker_host_preflight,
    );
    let enforcement_ready_count = rows
        .iter()
        .filter(|row| {
            row.proof_state == APP_GAME_PLATFORM_PROOF_SCOPED_WINDOWS_EXECUTION
                && row.host_capability_state == APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE
        })
        .count() as u64;

    AppGamePlatformProofStatusReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        read_model_id: APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL_ID.to_string(),
        generated_at: generated_at.0,
        source_read_model_ids: vec![proof::READ_MODEL_ID.to_string()],
        custody_label: APP_GAME_PLATFORM_PROOF_STATUS_CUSTODY_LABEL.to_string(),
        capability_status: APP_GAME_PLATFORM_PROOF_STATUS_CAPABILITY_PARTIAL.to_string(),
        returned: rows.len() as u64,
        host_visible_count: count_rows(
            &rows,
            TextValue(APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE),
        ),
        host_not_detected_count: count_rows(
            &rows,
            TextValue(APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED),
        ),
        local_runtime_not_applicable_count: count_rows(
            &rows,
            TextValue(APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_APPLICABLE),
        ),
        enforcement_ready_count,
        open_gap_count: rows.iter().map(|row| row.open_gaps.len() as u64).sum(),
        adapter_dispatch_claimed: false,
        broad_installed_app_blocking_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        child_device_delivery_claimed: false,
        private_diagnostics_claimed: false,
        rows,
    }
}

fn detect_platform_host_state() -> (HostCapabilitySignals, AppGameLinuxDockerHostPreflight) {
    let linux_docker_host_preflight = detect_linux_docker_host_preflight();
    let host_capabilities = HostCapabilitySignals::detect();
    (host_capabilities, linux_docker_host_preflight)
}

fn unavailable_platform_host_state() -> (HostCapabilitySignals, AppGameLinuxDockerHostPreflight) {
    (
        HostCapabilitySignals::unavailable(),
        unavailable_linux_docker_host_preflight(),
    )
}

pub fn app_game_platform_proof_status_payload(
    read_model: &AppGamePlatformProofStatusReadModel,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::CUSTODY_LABEL,
            LogFieldValue::String(read_model.custody_label.clone()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(read_model.capability_status.clone()),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.returned as f64),
        ),
        (
            constants::field::APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL,
            LogFieldValue::String(serialized_read_model(read_model).0),
        ),
    ])
}

fn platform_status_rows(
    generated_at: &GeneratedAtText,
    host_capabilities: &HostCapabilitySignals,
    linux_preflight: &LinuxForegroundSourcePreflight,
    linux_docker_host_preflight: &AppGameLinuxDockerHostPreflight,
) -> Vec<AppGamePlatformProofStatusRow> {
    vec![
        windows_status_row(generated_at),
        android_status_row(generated_at, host_capabilities),
        linux_status_row(
            generated_at,
            host_capabilities,
            linux_preflight,
            linux_docker_host_preflight,
        ),
        platform_not_applicable_status_row(
            generated_at,
            TextValue(APP_GAME_PARENT_PLATFORM_MACOS),
            TextValue(APP_GAME_PLATFORM_GAP_MACOS_ARTIFACTS),
        ),
        platform_not_applicable_status_row(
            generated_at,
            TextValue(APP_GAME_PARENT_PLATFORM_IOS),
            TextValue(APP_GAME_PLATFORM_GAP_IOS_ARTIFACTS),
        ),
    ]
}

fn windows_status_row(generated_at: &GeneratedAtText) -> AppGamePlatformProofStatusRow {
    platform_status_row(&PlatformStatusSpec {
        generated_at,
        platform: TextValue(APP_GAME_PARENT_PLATFORM_WINDOWS),
        proof_state: TextValue(APP_GAME_PLATFORM_PROOF_SCOPED_WINDOWS_EXECUTION),
        authority_state: TextValue(APP_GAME_PLATFORM_AUTHORITY_SCOPED_EXECUTION_ONLY),
        host_capability_state: TextValue(APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE),
        host_capability_evidence_refs: TextList(vec![
            proof::REF_ADAPTER_CAPABILITY_STATE.to_string(),
        ]),
        host_capability_probe_refs: TextList(vec![proof::REF_WINDOWS_HOST_LOCAL_PROBE.to_string()]),
        linux_docker_host_preflight: None,
        proof_refs: TextList(vec![
            proof::REF_APP_SESSION_EVIDENCE.to_string(),
            proof::REF_TIMER_STATE.to_string(),
        ]),
        open_gaps: TextList(vec![
            APP_GAME_PLATFORM_GAP_BROAD_BLOCKING.to_string(),
            APP_GAME_PLATFORM_GAP_PLATFORM_ENFORCEMENT.to_string(),
            APP_GAME_PLATFORM_GAP_CHILD_DELIVERY.to_string(),
        ]),
    })
}

fn android_status_row(
    generated_at: &GeneratedAtText,
    host_capabilities: &HostCapabilitySignals,
) -> AppGamePlatformProofStatusRow {
    let host_state = host_capabilities.android_state().0;
    platform_status_row(&PlatformStatusSpec {
        generated_at,
        platform: TextValue(APP_GAME_PARENT_PLATFORM_ANDROID),
        proof_state: if host_state == APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE {
            TextValue(APP_GAME_PLATFORM_PROOF_ANDROID_HOST_VISIBLE)
        } else {
            TextValue(APP_GAME_PLATFORM_PROOF_ANDROID_HOST_NOT_DETECTED)
        },
        authority_state: TextValue(APP_GAME_PLATFORM_AUTHORITY_VISIBILITY_ONLY),
        host_capability_state: TextValue(host_state),
        host_capability_evidence_refs: TextList(host_capabilities.android_evidence_refs().0),
        host_capability_probe_refs: TextList(host_capabilities.android_probe_refs().0),
        linux_docker_host_preflight: None,
        proof_refs: TextList(vec![
            proof::REF_ANDROID_ADB_HOST_TOOLCHAIN.to_string(),
            proof::REF_ANDROID_PHYSICAL_DEVICE_PROOF.to_string(),
            proof::REF_ANDROID_USAGE_EVENTS_FOREGROUND.to_string(),
        ]),
        open_gaps: TextList(vec![
            APP_GAME_PLATFORM_GAP_ANDROID_DEVICE_OWNER.to_string(),
            APP_GAME_PLATFORM_GAP_ANDROID_DURABLE_USAGE_REPLAY.to_string(),
            APP_GAME_PLATFORM_GAP_PLATFORM_ENFORCEMENT.to_string(),
            APP_GAME_PLATFORM_GAP_CHILD_DELIVERY.to_string(),
        ]),
    })
}

fn linux_status_row(
    generated_at: &GeneratedAtText,
    host_capabilities: &HostCapabilitySignals,
    linux_preflight: &LinuxForegroundSourcePreflight,
    linux_docker_host_preflight: &AppGameLinuxDockerHostPreflight,
) -> AppGamePlatformProofStatusRow {
    let host_state = host_capabilities.linux_state_for(linux_preflight).0;
    let mut proof_refs = vec![
        proof::REF_LINUX_WSL_HOST_TOOLCHAIN.to_string(),
        proof::REF_LINUX_WSLG_DISPLAY.to_string(),
        proof::REF_LINUX_WSLG_X11_SOCKET.to_string(),
        proof::REF_LINUX_WSLG_WAYLAND_SOCKET.to_string(),
    ];
    proof_refs.extend(host_capabilities.linux_proof_refs_for(linux_preflight).0);
    proof_refs.extend(linux_docker_host_preflight.proof_refs.clone());
    let mut open_gaps = vec![
        APP_GAME_PLATFORM_GAP_LINUX_NATIVE_SERVICE.to_string(),
        APP_GAME_PLATFORM_GAP_LINUX_FOREGROUND_CAPTURE.to_string(),
        APP_GAME_PLATFORM_GAP_LINUX_ROLLBACK.to_string(),
        APP_GAME_PLATFORM_GAP_PLATFORM_ENFORCEMENT.to_string(),
        APP_GAME_PLATFORM_GAP_CHILD_DELIVERY.to_string(),
    ];
    if linux_docker_host_preflight.state != APP_GAME_LINUX_DOCKER_PREFLIGHT_READY {
        open_gaps.push(APP_GAME_PLATFORM_GAP_LINUX_DOCKER_PREFLIGHT.to_string());
    }
    platform_status_row(&PlatformStatusSpec {
        generated_at,
        platform: TextValue(APP_GAME_PARENT_PLATFORM_LINUX),
        proof_state: if host_state == APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE {
            TextValue(APP_GAME_PLATFORM_PROOF_LINUX_HOST_VISIBLE)
        } else {
            TextValue(APP_GAME_PLATFORM_PROOF_LINUX_HOST_NOT_DETECTED)
        },
        authority_state: TextValue(APP_GAME_PLATFORM_AUTHORITY_VISIBILITY_ONLY),
        host_capability_state: TextValue(host_state),
        host_capability_evidence_refs: TextList(
            host_capabilities.linux_evidence_refs_for(linux_preflight).0,
        ),
        host_capability_probe_refs: TextList(
            host_capabilities.linux_probe_refs_for(linux_preflight).0,
        ),
        linux_docker_host_preflight: Some(linux_docker_host_preflight.clone()),
        proof_refs: TextList(proof_refs),
        open_gaps: TextList(open_gaps),
    })
}

fn platform_not_applicable_status_row(
    generated_at: &GeneratedAtText,
    platform: TextValue,
    platform_gap: TextValue,
) -> AppGamePlatformProofStatusRow {
    platform_status_row(&PlatformStatusSpec {
        generated_at,
        platform,
        proof_state: TextValue(APP_GAME_PLATFORM_PROOF_LOCAL_RUNTIME_NOT_APPLICABLE),
        authority_state: TextValue(APP_GAME_PLATFORM_AUTHORITY_NOT_LOCALLY_PROVABLE),
        host_capability_state: TextValue(APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_APPLICABLE),
        host_capability_evidence_refs: TextList(Vec::new()),
        host_capability_probe_refs: TextList(Vec::new()),
        linux_docker_host_preflight: None,
        proof_refs: TextList(vec![platform_gap.0.to_string()]),
        open_gaps: TextList(vec![
            platform_gap.0.to_string(),
            APP_GAME_PLATFORM_GAP_PLATFORM_ENFORCEMENT.to_string(),
            APP_GAME_PLATFORM_GAP_CHILD_DELIVERY.to_string(),
        ]),
    })
}

struct PlatformStatusSpec<'a> {
    generated_at: &'a GeneratedAtText,
    platform: TextValue,
    proof_state: TextValue,
    authority_state: TextValue,
    host_capability_state: TextValue,
    host_capability_evidence_refs: TextList,
    host_capability_probe_refs: TextList,
    linux_docker_host_preflight: Option<AppGameLinuxDockerHostPreflight>,
    proof_refs: TextList,
    open_gaps: TextList,
}

fn platform_status_row(spec: &PlatformStatusSpec<'_>) -> AppGamePlatformProofStatusRow {
    let mut row_id = String::from(APP_GAME_PLATFORM_PROOF_STATUS_ROW_ID_PREFIX);
    row_id.push_str(spec.platform.0);

    AppGamePlatformProofStatusRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id,
        platform: spec.platform.0.to_string(),
        proof_state: spec.proof_state.0.to_string(),
        authority_state: spec.authority_state.0.to_string(),
        host_capability_state: spec.host_capability_state.0.to_string(),
        host_capability_evidence_refs: spec.host_capability_evidence_refs.0.clone(),
        host_capability_probe_refs: spec.host_capability_probe_refs.0.clone(),
        linux_docker_host_preflight: spec.linux_docker_host_preflight.clone(),
        product_meanings: vec![
            APP_GAME_ADAPTER_PRODUCT_NATIVE_APP.to_string(),
            APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME.to_string(),
        ],
        proof_refs: spec.proof_refs.0.clone(),
        open_gaps: spec.open_gaps.0.clone(),
        adapter_dispatch_claimed: false,
        broad_installed_app_blocking_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        child_device_delivery_claimed: false,
        private_diagnostics_claimed: false,
        last_checked_at: spec.generated_at.0.clone(),
    }
}

fn serialized_read_model(
    read_model: &AppGamePlatformProofStatusReadModel,
) -> SerializedReadModelText {
    match serde_json::to_string(read_model) {
        Ok(json) => SerializedReadModelText(json),
        Err(_error) => SerializedReadModelText(constants::value::EMPTY.to_string()),
    }
}

fn count_rows(rows: &[AppGamePlatformProofStatusRow], state: TextValue) -> u64 {
    rows.iter()
        .filter(|row| row.host_capability_state == state.0)
        .count() as u64
}
