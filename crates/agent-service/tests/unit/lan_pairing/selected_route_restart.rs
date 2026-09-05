use crate::test_text::TestText;

use std::{
    fs::remove_file,
    sync::atomic::{AtomicUsize, Ordering},
};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use ocentra_parent_agent_protocol::transport::AgentMessageTarget;
use ocentra_parent_agent_protocol::transport::AgentRoute;

use crate::{
    app::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test},
    lan_pairing_test_assertions::{
        assert_persistent_status_support_surface, assert_rejection, assert_rejection_with_audit,
        assert_status_selected_route_trust, assert_status_selection,
    },
    lan_pairing_test_commands::{
        command_for_target, health_command, intent_payload, pairing_command, proof_payload,
        route_select_command, serialize_command,
    },
};

#[tokio::test]
async fn lan_pairing_persistent_registry_remains_unpaired_after_uncomposed_selection_restart() {
    let mut path = std::env::temp_dir();
    path.push(temp_registry_name().0);
    path.set_extension(constants::lan_pairing::REGISTRY_FILE_EXTENSION);
    let _ = remove_file(&path);
    let runtime = LanPairingRuntime::persistent_json(&path);
    let pairing_event = signed_pairing(runtime.clone()).await;
    let route_selected = signed_route_select(runtime).await;
    let restarted_runtime = LanPairingRuntime::persistent_json(&path);
    let restarted_status = loopback_lan_status(restarted_runtime.clone()).await;
    let accepted_after_restart = old_signed_control(restarted_runtime).await;
    let _ = remove_file(&path);

    assert_eq!(
        restarted_status.event,
        AgentEventName::AgentLanPairingStatusReported
    );
    assert_rejection_with_audit(
        &pairing_event,
        constants::value::LAN_REASON_ANONYMOUS,
        constants::value::LAN_AUDIT_PAIRING_PROOF_REJECTED,
    );
    assert_rejection(
        &route_selected,
        constants::value::LAN_REASON_SIGNED_CHILD_AGENT_CONTEXT_UNAVAILABLE,
    );
    assert_persistent_status_support_surface(&restarted_status);
    assert_status_selection(
        &restarted_status,
        constants::value::LAN_AUTH_UNPAIRED,
        constants::value::EMPTY,
        constants::value::EMPTY,
        constants::value::EMPTY,
    );
    assert_status_selected_route_trust(
        &restarted_status,
        constants::value::EMPTY,
        constants::value::EMPTY,
        constants::value::EMPTY,
        constants::value::EMPTY,
    );
    assert_rejection(
        &accepted_after_restart,
        constants::value::LAN_REASON_ANONYMOUS,
    );
}

async fn signed_pairing(runtime: LanPairingRuntime) -> AgentEventEnvelope {
    handle_command_text_for_test(
        serialize_command(pairing_command(proof_payload())),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await
}

async fn signed_route_select(runtime: LanPairingRuntime) -> AgentEventEnvelope {
    handle_command_text_for_test(
        serialize_command(route_select_command(intent_payload(
            constants::lan_pairing::SELECT_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await
}

async fn loopback_lan_status(runtime: LanPairingRuntime) -> AgentEventEnvelope {
    handle_command_text_for_test(serialize_command(loopback_status_command()), runtime, None).await
}

fn loopback_status_command() -> ocentra_parent_agent_protocol::transport::AgentCommandEnvelope {
    command_for_target(
        ocentra_parent_agent_protocol::transport::AgentCommandName::AgentLanPairingStatusGet,
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
        serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await
}

fn temp_registry_name() -> TestText {
    static REGISTRY_COUNTER: AtomicUsize = AtomicUsize::new(0);
    let mut name = String::from(constants::lan_pairing::REGISTRY_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push_str(&REGISTRY_COUNTER.fetch_add(1, Ordering::Relaxed).to_string());
    TestText(name)
}
