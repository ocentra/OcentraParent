use std::{
    collections::HashMap,
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
    dispatch_parent_ui_action, load_parent_route_snapshot, load_parent_subscription_event,
};
use ocentra_schema::parent_ui_bridge::{
    ParentRouteContext, ParentRouteId, ParentRouteSnapshot, ParentSubscriptionEvent,
    ParentUiAction, ParentUiActionResult,
};
use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

const SERVICE_CONNECT_TIMEOUT_MS: u64 = 250;
const PARENT_ROUTE_SUBSCRIPTION_POLL_INTERVAL_MS: u64 = 1000;

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
struct ParentRouteSubscriptionRegistry {
    inner: Arc<ParentRouteSubscriptionRegistryInner>,
}

#[derive(Default)]
struct ParentRouteSubscriptionRegistryInner {
    next_id: AtomicU64,
    subscriptions: Mutex<HashMap<String, Arc<AtomicBool>>>,
}

impl ParentRouteSubscriptionRegistry {
    fn register(&self) -> (String, Arc<AtomicBool>) {
        let subscription_id = self
            .inner
            .next_id
            .fetch_add(1, Ordering::SeqCst)
            .to_string();
        let active = Arc::new(AtomicBool::new(true));
        self.inner
            .subscriptions
            .lock()
            .unwrap_or_else(|_| unreachable!("parent route subscriptions lock remains available"))
            .insert(subscription_id.clone(), Arc::clone(&active));
        (subscription_id, active)
    }

    fn unregister(&self, subscription_id: &str) -> bool {
        self.inner
            .subscriptions
            .lock()
            .unwrap_or_else(|_| unreachable!("parent route subscriptions lock remains available"))
            .remove(subscription_id)
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
    load_parent_route_snapshot(route, context)
}

#[tauri::command]
fn parent_dispatch(action: ParentUiAction) -> ParentUiActionResult {
    dispatch_parent_ui_action(action)
}

#[tauri::command]
fn parent_subscribe_route(
    app: AppHandle,
    registry: State<'_, ParentRouteSubscriptionRegistry>,
    route: ParentRouteId,
    context: Option<ParentRouteContext>,
) -> Result<String, String> {
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
    registry: State<'_, ParentRouteSubscriptionRegistry>,
    subscription_id: String,
) -> bool {
    registry.unregister(subscription_id.as_str())
}

pub fn run() {
    let result = tauri::Builder::default()
        .manage(ParentRouteSubscriptionRegistry::default())
        .invoke_handler(tauri::generate_handler![
            parent_platform_proof_state,
            parent_load_route,
            parent_dispatch,
            parent_subscribe_route,
            parent_unsubscribe_route
        ])
        .run(tauri::generate_context!());
    if let Err(error) = result {
        panic!("{error}");
    }
}

fn spawn_parent_route_subscription(
    app: AppHandle,
    registry: ParentRouteSubscriptionRegistry,
    subscription_id: String,
    route: ParentRouteId,
    context: Option<ParentRouteContext>,
    active: Arc<AtomicBool>,
) {
    thread::spawn(move || {
        let mut last_snapshot = Some(load_parent_route_snapshot(route.clone(), context.clone()));
        while active.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(
                PARENT_ROUTE_SUBSCRIPTION_POLL_INTERVAL_MS,
            ));
            if !active.load(Ordering::SeqCst) {
                break;
            }
            let event = load_parent_subscription_event(route.clone(), context.clone());
            if last_snapshot.as_ref() == Some(&event.snapshot) {
                continue;
            }
            if emit_parent_route_subscription_event(&app, &subscription_id, &event).is_err() {
                break;
            }
            last_snapshot = Some(event.snapshot);
        }
        let _ = registry.unregister(subscription_id.as_str());
    });
}

fn emit_parent_route_subscription_event(
    app: &AppHandle,
    subscription_id: &str,
    event: &ParentSubscriptionEvent,
) -> Result<(), String> {
    app.emit(
        parent_route_subscription_event_name(subscription_id).as_str(),
        event.clone(),
    )
    .map_err(|error| {
        format!("parent desktop route subscription emit failed for {subscription_id}: {error}")
    })
}

fn parent_route_subscription_event_name(subscription_id: &str) -> String {
    format!("parent-route-subscription-{subscription_id}")
}

fn configured_agent_address() -> String {
    std::env::var(constants::env_var::AGENT_ADDR)
        .unwrap_or_else(|_| constants::bind::DEFAULT_AGENT_ADDR.to_string())
}

fn parent_platform_proof_state_for_address(
    agent_address: String,
) -> ParentDesktopPlatformProofState {
    let service_connects = agent_service_connects(&agent_address);
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

fn agent_service_connects(agent_address: &str) -> bool {
    agent_address
        .parse::<SocketAddr>()
        .ok()
        .and_then(|address| {
            TcpStream::connect_timeout(&address, Duration::from_millis(SERVICE_CONNECT_TIMEOUT_MS))
                .ok()
        })
        .is_some()
}

fn parent_desktop_device_role_state() -> DeviceRoleRuntimeReadModel {
    DeviceRoleRuntimeReadModel {
        schema_version: constants::lan_pairing::SCHEMA_VERSION_TEXT.to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    #[test]
    fn parent_platform_proof_state_uses_rust_service_connection_for_package_runtime() {
        let state = parent_platform_proof_state_for_address(
            constants::test_network::LOOPBACK_ANY_PORT.to_string(),
        );

        assert_eq!(
            state.service_state,
            constants::value::PARENT_DESKTOP_SERVICE_UNAVAILABLE
        );
        assert_eq!(state.service_health_endpoint, constants::endpoint::HEALTH);
        assert_eq!(
            state.runtime_readiness_state,
            constants::value::PARENT_DESKTOP_RUNTIME_DEGRADED
        );
        assert_eq!(
            state.backend_kind,
            constants::value::PARENT_DESKTOP_BACKEND_RUST_SERVICE
        );
        assert_eq!(state.route_state, DeviceRuntimeRouteState::LocalNetwork);
        assert_eq!(
            state.route_source_state,
            DeviceRuntimeRouteState::LocalNetwork
        );
        assert_eq!(
            state.lan_ai_provider_state,
            DeviceRuntimeAiProviderState::Degraded
        );
        assert_eq!(
            state.degraded_source_state,
            DeviceRuntimeAiProviderState::Degraded
        );
        assert_eq!(
            state.activity_adapter_state,
            constants::value::PARENT_DESKTOP_SERVICE_UNAVAILABLE
        );
        assert_eq!(
            state.parent_assistant_provider_state,
            DeviceRuntimeAiProviderState::Degraded
        );
        assert_eq!(
            state.device_role_state.local_ai_runtime_claim,
            DeviceRuntimeLocalAiClaim::SharedPhysicalDeviceSingleton
        );
        assert_eq!(
            state.package_frontend_state,
            constants::value::PARENT_DESKTOP_FRONTEND_BUILT_PORTAL_DIST
        );
        assert_eq!(
            state.hmr_backend_state,
            constants::value::PARENT_DESKTOP_HMR_BACKEND_NOT_USED
        );
        assert_eq!(
            state.process_ownership_state,
            constants::value::PARENT_DESKTOP_PROCESS_OWNER_SHELL_ONLY
        );
        assert_eq!(
            state.controller_route_state,
            constants::value::PARENT_DESKTOP_CONTROLLER_ROUTE_ACTIVE_CONTROLLER
        );
        assert_eq!(
            state.observer_read_only_state,
            constants::value::PARENT_DESKTOP_OBSERVER_READ_ONLY
        );
        assert_eq!(
            state.source_custody_state,
            constants::value::PARENT_DESKTOP_SOURCE_CUSTODY_LIVE_LOCAL_NETWORK
        );
        assert_eq!(
            state.relay_route_state,
            constants::value::PARENT_DESKTOP_RELAY_ROUTE_UNAVAILABLE
        );
        assert_eq!(
            state.parent_cache_state,
            constants::value::PARENT_DESKTOP_PARENT_CACHE_UNAVAILABLE
        );
        assert_eq!(
            state.parent_storage_state,
            constants::value::PARENT_DESKTOP_PARENT_STORAGE_UNAVAILABLE
        );
        assert_eq!(
            state.service_launch_owner_state,
            constants::value::PARENT_DESKTOP_SERVICE_LAUNCH_OWNER_PACKAGE_SERVICE
        );
        assert_eq!(
            state.service_launch_strategy_state,
            constants::value::PARENT_DESKTOP_SERVICE_LAUNCH_STRATEGY_CONNECT_OR_DEGRADE
        );
        assert_eq!(state.service_connect_timeout_ms, SERVICE_CONNECT_TIMEOUT_MS);
        assert_eq!(
            state.package_service_manager_state,
            constants::value::PARENT_DESKTOP_PACKAGE_SERVICE_AUTO_START
        );
        assert_eq!(
            state.package_health_probe_state,
            constants::value::PARENT_DESKTOP_PACKAGE_HEALTH_PROBE_REQUIRED
        );
        assert_eq!(
            state.port_ownership_state,
            constants::value::PARENT_DESKTOP_PORT_OWNERSHIP_FIXED_LOOPBACK
        );
        assert_eq!(
            state.port_conflict_policy_state,
            constants::value::PARENT_DESKTOP_PORT_CONFLICT_POLICY_NO_FOREIGN_RECLAIM
        );
        assert_eq!(
            state.blank_window_regression_state,
            constants::value::PARENT_DESKTOP_BLANK_WINDOW_GUARD_FRONTEND_DIST
        );
        assert_eq!(
            state.package_preview_state,
            constants::value::PARENT_DESKTOP_PACKAGE_PREVIEW_UNSIGNED
        );
        assert_eq!(
            state.update_channel_state,
            constants::value::PARENT_DESKTOP_UPDATE_CHANNEL_SCAFFOLD
        );
        assert_eq!(
            state.rollback_state,
            constants::value::PARENT_DESKTOP_ROLLBACK_UNAVAILABLE
        );
        assert_eq!(
            state.signing_state,
            constants::value::PARENT_DESKTOP_SIGNING_MANUAL_REQUIRED
        );
        assert_eq!(
            state.notarization_state,
            constants::value::PARENT_DESKTOP_NOTARIZATION_MANUAL_REQUIRED
        );
        assert_eq!(
            state.store_distribution_state,
            constants::value::PARENT_DESKTOP_STORE_DISTRIBUTION_MANUAL_REQUIRED
        );
        assert_eq!(
            state.support_diagnostics_state,
            constants::value::PARENT_DESKTOP_SUPPORT_DIAGNOSTICS_REDACTED
        );
        assert_eq!(
            state.support_redaction_state,
            constants::value::PARENT_DESKTOP_SUPPORT_OUTPUT_ALLOWED_FIELDS
        );
        assert_eq!(
            state.platform_matrix_state,
            constants::value::PARENT_DESKTOP_PLATFORM_MATRIX_SPLIT_PROOF_ROWS
        );
        assert_eq!(
            state.release_branch_state,
            constants::value::PARENT_DESKTOP_RELEASE_BRANCH_PRODUCTION_PROMOTION_REQUIRED
        );
        assert_eq!(
            state.artifact_proof_state,
            constants::value::PARENT_DESKTOP_ARTIFACT_PROOF_CI_PREVIEW
        );
    }

    #[test]
    fn parent_platform_proof_state_reports_ready_when_rust_service_socket_accepts() {
        let listener =
            TcpListener::bind((constants::test_network::LOOPBACK_IP, 0)).expect("bind listener");
        let state = parent_platform_proof_state_for_address(
            listener.local_addr().expect("listener address").to_string(),
        );

        assert_eq!(
            state.service_state,
            constants::value::PARENT_DESKTOP_SERVICE_CONNECTED
        );
        assert_eq!(
            state.runtime_readiness_state,
            constants::value::PARENT_DESKTOP_RUNTIME_READY
        );
        assert_eq!(
            state.activity_adapter_state,
            constants::value::PARENT_DESKTOP_SERVICE_CONNECTED
        );
    }

    #[test]
    fn parent_route_subscription_registry_unregisters_active_subscriptions() {
        let registry = ParentRouteSubscriptionRegistry::default();
        let (subscription_id, active) = registry.register();

        assert!(active.load(Ordering::SeqCst));
        assert!(registry.unregister(subscription_id.as_str()));
        assert!(!active.load(Ordering::SeqCst));
        assert!(!registry.unregister(subscription_id.as_str()));
    }

    #[test]
    fn parent_route_subscription_event_name_uses_stable_prefix() {
        assert_eq!(
            parent_route_subscription_event_name("42"),
            "parent-route-subscription-42"
        );
    }
}
