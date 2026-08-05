use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    net::{SocketAddr, TcpStream},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use ocentra_parent_agent_protocol::{
    constants, DeviceRoleRuntimeReadModel, DeviceRuntimeAiProviderState, DeviceRuntimeLocalAiClaim,
    DeviceRuntimeRole, DeviceRuntimeRoleEntry, DeviceRuntimeRoleState, DeviceRuntimeRouteState,
    DeviceRuntimeSurface, LanPairingParentAuthority,
};
use ocentra_parent_runtime_core::parent_ui_bridge::{
    dispatch_parent_ui_action_with_device_trust, load_parent_route_snapshot,
};
use ocentra_parent_runtime_core::{
    device_trust_bootstrap_runtime::ParentDeviceTrustCommandFacade,
    parent_ui_bridge::lan_replay_rejection_episode::ParentRouteSubscriptionLoadState,
};
use ocentra_schema::parent_ui_bridge::{
    ParentRouteContext, ParentRouteId, ParentRouteSnapshot, ParentSubscriptionEvent,
    ParentUiAction, ParentUiActionResult, PARENT_ROUTE_SUBSCRIPTION_EVENT_PREFIX,
    PARENT_ROUTE_SUBSCRIPTION_POLL_INTERVAL_MS,
};
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, State as TauriState};

use self::parent_route_subscription_delivery::{
    deliver_parent_route_subscription_event, ParentRouteSubscriptionDeliveryState,
};

pub mod parent_route_subscription_delivery;

const SERVICE_CONNECT_TIMEOUT_MS: u64 = 250;
const PARENT_DEVICE_TRUST_STORAGE_DIRECTORY: &str = "device-trust";

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ParentRouteSubscriptionId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ParentDesktopAgentAddress(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ParentRouteSubscriptionEventName(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ParentDesktopCommandError(String);

impl ParentDesktopCommandError {
    fn from_tauri_error(error: tauri::Error) -> Self {
        Self(error.to_string())
    }
}

impl Display for ParentDesktopCommandError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ParentDesktopCommandError {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentDesktopPlatformProofState {
    service_state: String,
    agent_address: String,
    service_health_endpoint: String,
    runtime_readiness_state: String,
    controller_lease_state: String,
    device_role_state: DeviceRoleRuntimeReadModel,
    activity_adapter_state: String,
    parent_assistant_provider_state: DeviceRuntimeAiProviderState,
    route_state: DeviceRuntimeRouteState,
    route_source_state: DeviceRuntimeRouteState,
    lan_ai_provider_state: DeviceRuntimeAiProviderState,
    degraded_source_state: DeviceRuntimeAiProviderState,
    backend_kind: String,
    package_frontend_state: String,
    hmr_backend_state: String,
    process_ownership_state: String,
    controller_route_state: String,
    observer_read_only_state: String,
    source_custody_state: String,
    relay_route_state: String,
    parent_cache_state: String,
    parent_storage_state: String,
    service_launch_owner_state: String,
    service_launch_strategy_state: String,
    service_connect_timeout_ms: u64,
    package_service_manager_state: String,
    package_health_probe_state: String,
    port_ownership_state: String,
    port_conflict_policy_state: String,
    blank_window_regression_state: String,
    package_preview_state: String,
    update_channel_state: String,
    rollback_state: String,
    signing_state: String,
    notarization_state: String,
    store_distribution_state: String,
    support_diagnostics_state: String,
    support_redaction_state: String,
    platform_matrix_state: String,
    release_branch_state: String,
    artifact_proof_state: String,
}

#[derive(Clone, Default)]
pub struct ParentRouteSubscriptionRegistry {
    inner: Arc<ParentRouteSubscriptionRegistryInner>,
}

pub struct ParentDeviceTrustCommandState(ParentDeviceTrustCommandFacade);

#[derive(Default)]
struct ParentRouteSubscriptionRegistryInner {
    next_id: AtomicU64,
    subscriptions: Mutex<HashMap<ParentRouteSubscriptionId, Arc<AtomicBool>>>,
}

impl ParentRouteSubscriptionRegistry {
    pub fn register(&self) -> (ParentRouteSubscriptionId, Arc<AtomicBool>) {
        let subscription_id = ParentRouteSubscriptionId(
            self.inner
                .next_id
                .fetch_add(1, Ordering::SeqCst)
                .to_string(),
        );
        let active = Arc::new(AtomicBool::new(true));
        let mut subscriptions = self
            .inner
            .subscriptions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        subscriptions.insert(subscription_id.clone(), Arc::clone(&active));
        (subscription_id, active)
    }

    pub fn unregister(&self, subscription_id: &ParentRouteSubscriptionId) -> bool {
        let removed = self
            .inner
            .subscriptions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .remove(subscription_id);

        removed
            .map(|active| {
                active.store(false, Ordering::SeqCst);
                true
            })
            .unwrap_or(false)
    }
}

#[tauri::command]
fn parent_platform_proof_state() -> ParentDesktopPlatformProofState {
    parent_platform_proof_state_for_address(configured_agent_address())
}

#[tauri::command]
fn parent_load_route(
    route: ParentRouteId,
    context: Option<ParentRouteContext>,
) -> ParentRouteSnapshot {
    load_parent_route_snapshot(route, context.as_ref())
}

#[tauri::command]
fn parent_dispatch(
    device_trust: TauriState<'_, ParentDeviceTrustCommandState>,
    action: ParentUiAction,
) -> ParentUiActionResult {
    dispatch_parent_ui_action_with_device_trust(&action, &device_trust.0)
}

#[tauri::command]
fn parent_subscribe_route(
    app: AppHandle,
    registry: TauriState<'_, ParentRouteSubscriptionRegistry>,
    route: ParentRouteId,
    context: Option<ParentRouteContext>,
) -> Result<ParentRouteSubscriptionId, ParentDesktopCommandError> {
    let registry = registry.inner().clone();
    let (subscription_id, active) = registry.register();
    spawn_parent_route_subscription(
        app,
        registry,
        subscription_id.clone(),
        route,
        context,
        active,
    );
    Ok(subscription_id)
}

#[tauri::command]
fn parent_unsubscribe_route(
    registry: TauriState<'_, ParentRouteSubscriptionRegistry>,
    subscription_id: ParentRouteSubscriptionId,
) -> bool {
    registry.unregister(&subscription_id)
}

pub fn run() -> Result<(), ParentDesktopCommandError> {
    tauri::Builder::default()
        .setup(|app| {
            let root = app
                .path()
                .app_local_data_dir()
                .map_err(|error| std::io::Error::other(error.to_string()))?
                .join(PARENT_DEVICE_TRUST_STORAGE_DIRECTORY);
            let device_trust = ParentDeviceTrustCommandFacade::open(root)
                .map_err(|error| std::io::Error::other(format!("{error:?}")))?;
            app.manage(ParentDeviceTrustCommandState(device_trust));
            Ok(())
        })
        .manage(ParentRouteSubscriptionRegistry::default())
        .invoke_handler(tauri::generate_handler![
            parent_platform_proof_state,
            parent_load_route,
            parent_dispatch,
            parent_subscribe_route,
            parent_unsubscribe_route
        ])
        .run(tauri::generate_context!())
        .map_err(ParentDesktopCommandError::from_tauri_error)
}

fn spawn_parent_route_subscription(
    app: AppHandle,
    registry: ParentRouteSubscriptionRegistry,
    subscription_id: ParentRouteSubscriptionId,
    route: ParentRouteId,
    context: Option<ParentRouteContext>,
    active: Arc<AtomicBool>,
) {
    thread::spawn(move || {
        let mut load_state = ParentRouteSubscriptionLoadState::default();
        let mut delivery_state = ParentRouteSubscriptionDeliveryState::new(
            load_parent_route_snapshot(route.clone(), context.as_ref()),
        );
        while active.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(
                PARENT_ROUTE_SUBSCRIPTION_POLL_INTERVAL_MS,
            ));
            if !active.load(Ordering::SeqCst) {
                break;
            }
            let event = load_state.load(route.clone(), context.as_ref());
            if deliver_parent_route_subscription_event(&mut delivery_state, &event, |event| {
                emit_parent_route_subscription_event(&app, &subscription_id, event)
            })
            .is_err()
            {
                break;
            }
        }
        let _ = registry.unregister(&subscription_id);
    });
}

fn emit_parent_route_subscription_event(
    app: &AppHandle,
    subscription_id: &ParentRouteSubscriptionId,
    event: &ParentSubscriptionEvent,
) -> Result<(), ParentDesktopCommandError> {
    let event_name = parent_route_subscription_event_name(subscription_id);
    app.emit(event_name.0.as_str(), event.clone())
        .map_err(ParentDesktopCommandError::from_tauri_error)
}

fn configured_agent_address() -> ParentDesktopAgentAddress {
    ParentDesktopAgentAddress(
        std::env::var(constants::env_var::AGENT_ADDR)
            .unwrap_or_else(|_| constants::bind::DEFAULT_AGENT_ADDR.to_string()),
    )
}

pub fn parent_platform_proof_state_for_address(
    agent_address: ParentDesktopAgentAddress,
) -> ParentDesktopPlatformProofState {
    let service_connects = agent_service_connects(&agent_address);
    let agent_address = agent_address.0;
    let service_state = if service_connects {
        constants::value::PARENT_DESKTOP_SERVICE_CONNECTED
    } else {
        constants::value::PARENT_DESKTOP_SERVICE_UNAVAILABLE
    };
    let runtime_readiness_state = if service_connects {
        constants::value::PARENT_DESKTOP_RUNTIME_READY
    } else {
        constants::value::PARENT_DESKTOP_RUNTIME_DEGRADED
    };
    let device_role_state = parent_desktop_device_role_state();
    ParentDesktopPlatformProofState {
        service_state: service_state.to_string(),
        agent_address,
        service_health_endpoint: constants::endpoint::HEALTH.to_string(),
        runtime_readiness_state: runtime_readiness_state.to_string(),
        controller_lease_state: constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER
            .to_string(),
        activity_adapter_state: service_state.to_string(),
        parent_assistant_provider_state: device_role_state.lan_ai_provider_state.clone(),
        route_state: device_role_state.route_state.clone(),
        route_source_state: device_role_state.route_state.clone(),
        lan_ai_provider_state: device_role_state.lan_ai_provider_state.clone(),
        degraded_source_state: device_role_state.lan_ai_provider_state.clone(),
        device_role_state,
        backend_kind: constants::value::PARENT_DESKTOP_BACKEND_RUST_SERVICE.to_string(),
        package_frontend_state: constants::value::PARENT_DESKTOP_FRONTEND_BUILT_PORTAL_DIST
            .to_string(),
        hmr_backend_state: constants::value::PARENT_DESKTOP_HMR_BACKEND_NOT_USED.to_string(),
        process_ownership_state: constants::value::PARENT_DESKTOP_PROCESS_OWNER_SHELL_ONLY
            .to_string(),
        controller_route_state: constants::value::PARENT_DESKTOP_CONTROLLER_ROUTE_ACTIVE_CONTROLLER
            .to_string(),
        observer_read_only_state: constants::value::PARENT_DESKTOP_OBSERVER_READ_ONLY.to_string(),
        source_custody_state: constants::value::PARENT_DESKTOP_SOURCE_CUSTODY_LIVE_LOCAL_NETWORK
            .to_string(),
        relay_route_state: constants::value::PARENT_DESKTOP_RELAY_ROUTE_UNAVAILABLE.to_string(),
        parent_cache_state: constants::value::PARENT_DESKTOP_PARENT_CACHE_UNAVAILABLE.to_string(),
        parent_storage_state: constants::value::PARENT_DESKTOP_PARENT_STORAGE_UNAVAILABLE
            .to_string(),
        service_launch_owner_state:
            constants::value::PARENT_DESKTOP_SERVICE_LAUNCH_OWNER_PACKAGE_SERVICE.to_string(),
        service_launch_strategy_state:
            constants::value::PARENT_DESKTOP_SERVICE_LAUNCH_STRATEGY_CONNECT_OR_DEGRADE.to_string(),
        service_connect_timeout_ms: SERVICE_CONNECT_TIMEOUT_MS,
        package_service_manager_state: constants::value::PARENT_DESKTOP_PACKAGE_SERVICE_AUTO_START
            .to_string(),
        package_health_probe_state: constants::value::PARENT_DESKTOP_PACKAGE_HEALTH_PROBE_REQUIRED
            .to_string(),
        port_ownership_state: constants::value::PARENT_DESKTOP_PORT_OWNERSHIP_FIXED_LOOPBACK
            .to_string(),
        port_conflict_policy_state:
            constants::value::PARENT_DESKTOP_PORT_CONFLICT_POLICY_NO_FOREIGN_RECLAIM.to_string(),
        blank_window_regression_state:
            constants::value::PARENT_DESKTOP_BLANK_WINDOW_GUARD_FRONTEND_DIST.to_string(),
        package_preview_state: constants::value::PARENT_DESKTOP_PACKAGE_PREVIEW_UNSIGNED
            .to_string(),
        update_channel_state: constants::value::PARENT_DESKTOP_UPDATE_CHANNEL_SCAFFOLD.to_string(),
        rollback_state: constants::value::PARENT_DESKTOP_ROLLBACK_UNAVAILABLE.to_string(),
        signing_state: constants::value::PARENT_DESKTOP_SIGNING_MANUAL_REQUIRED.to_string(),
        notarization_state: constants::value::PARENT_DESKTOP_NOTARIZATION_MANUAL_REQUIRED
            .to_string(),
        store_distribution_state:
            constants::value::PARENT_DESKTOP_STORE_DISTRIBUTION_MANUAL_REQUIRED.to_string(),
        support_diagnostics_state: constants::value::PARENT_DESKTOP_SUPPORT_DIAGNOSTICS_REDACTED
            .to_string(),
        support_redaction_state: constants::value::PARENT_DESKTOP_SUPPORT_OUTPUT_ALLOWED_FIELDS
            .to_string(),
        platform_matrix_state: constants::value::PARENT_DESKTOP_PLATFORM_MATRIX_SPLIT_PROOF_ROWS
            .to_string(),
        release_branch_state:
            constants::value::PARENT_DESKTOP_RELEASE_BRANCH_PRODUCTION_PROMOTION_REQUIRED
                .to_string(),
        artifact_proof_state: constants::value::PARENT_DESKTOP_ARTIFACT_PROOF_CI_PREVIEW
            .to_string(),
    }
}

pub fn agent_service_connects(agent_address: &ParentDesktopAgentAddress) -> bool {
    agent_address
        .0
        .parse::<SocketAddr>()
        .ok()
        .and_then(|address| {
            TcpStream::connect_timeout(&address, Duration::from_millis(SERVICE_CONNECT_TIMEOUT_MS))
                .ok()
        })
        .is_some()
}

pub fn parent_route_subscription_event_name(
    subscription_id: &ParentRouteSubscriptionId,
) -> ParentRouteSubscriptionEventName {
    let mut event_name = String::from(PARENT_ROUTE_SUBSCRIPTION_EVENT_PREFIX);
    event_name.push_str(subscription_id.0.as_str());
    ParentRouteSubscriptionEventName(event_name)
}

fn parent_desktop_device_role_state() -> DeviceRoleRuntimeReadModel {
    DeviceRoleRuntimeReadModel {
        schema_version: constants::lan_pairing::SCHEMA_VERSION_TEXT.into(),
        physical_device_id: constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string(),
        surface: DeviceRuntimeSurface::ParentDesktop,
        platform: constants::local_ai_runtime::PLATFORM_OS_WINDOWS.to_string(),
        roles: vec![
            role_entry(DeviceRuntimeRole::ParentController),
            role_entry(DeviceRuntimeRole::ChildAgent),
            role_entry(DeviceRuntimeRole::AiProvider),
        ],
        primary_role: DeviceRuntimeRole::ParentController,
        controller_lease_id: Some(constants::lan_pairing::CONTROLLER_LEASE_ID.to_string()),
        parent_authority: Some(LanPairingParentAuthority::ActiveController),
        selected_route_id: Some(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string()),
        route_state: DeviceRuntimeRouteState::LocalNetwork,
        lan_ai_provider_state: DeviceRuntimeAiProviderState::Degraded,
        local_ai_runtime_claim: DeviceRuntimeLocalAiClaim::SharedPhysicalDeviceSingleton,
        updated_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
    }
}

fn role_entry(role: DeviceRuntimeRole) -> DeviceRuntimeRoleEntry {
    DeviceRuntimeRoleEntry {
        role,
        state: DeviceRuntimeRoleState::Implemented,
    }
}
