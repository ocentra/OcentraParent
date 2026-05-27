use std::{
    fs::remove_file,
    sync::atomic::{AtomicUsize, Ordering},
};

use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentEventEnvelope, AgentEventName, AgentMessageTarget,
    AgentRoute, LogFields,
};

use crate::{
    lan_pairing::LanPairingRuntime,
    lan_pairing_test_support::{
        assert_accepted_control, assert_persistent_selected_route_support_surface,
        assert_status_selection, command_for_target, health_command, intent_payload,
        pairing_command, proof_payload, route_select_command, serialize_command,
    },
    websocket::handle_command_text_for_test,
};

#[tokio::test]
async fn lan_pairing_persistent_registry_recovers_selected_route_after_restart() {
    let path = temp_registry_path();
    let _ = remove_file(&path);
    let runtime = LanPairingRuntime::persistent_json(&path);
    let _ = signed_pairing(runtime.clone()).await;
    let _ = signed_route_select(runtime).await;
    let restarted_runtime = LanPairingRuntime::persistent_json(&path);
    let restarted_status = loopback_lan_status(restarted_runtime.clone()).await;
    let accepted_after_restart = old_signed_control(restarted_runtime).await;
    let _ = remove_file(&path);

    assert_eq!(
        restarted_status.event,
        AgentEventName::AgentLanPairingStatusReported
    );
    assert_persistent_selected_route_support_surface(&restarted_status);
    assert_status_selection(
        &restarted_status,
        constants::value::LAN_AUTH_PAIRED,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
        constants::lan_pairing::CHILD_DEVICE_ID,
    );
    assert_accepted_control(&accepted_after_restart);
}

async fn signed_pairing(runtime: LanPairingRuntime) -> AgentEventEnvelope {
    handle_command_text_for_test(
        &serialize_command(pairing_command(proof_payload())),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await
}

async fn signed_route_select(runtime: LanPairingRuntime) -> AgentEventEnvelope {
    handle_command_text_for_test(
        &serialize_command(route_select_command(intent_payload(
            constants::lan_pairing::SELECT_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await
}

async fn loopback_lan_status(runtime: LanPairingRuntime) -> AgentEventEnvelope {
    handle_command_text_for_test(&serialize_command(loopback_status_command()), runtime, None).await
}

fn loopback_status_command() -> ocentra_parent_agent_protocol::AgentCommandEnvelope {
    command_for_target(
        ocentra_parent_agent_protocol::AgentCommandName::AgentLanPairingStatusGet,
        AgentMessageTarget {
            device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        LogFields::new(),
    )
}

async fn old_signed_control(runtime: LanPairingRuntime) -> AgentEventEnvelope {
    handle_command_text_for_test(
        &serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await
}

fn temp_registry_path() -> std::path::PathBuf {
    static REGISTRY_COUNTER: AtomicUsize = AtomicUsize::new(0);
    let mut name = String::from(constants::lan_pairing::REGISTRY_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push_str(&REGISTRY_COUNTER.fetch_add(1, Ordering::Relaxed).to_string());
    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::lan_pairing::REGISTRY_FILE_EXTENSION);
    path
}
