use ocentra_parent_agent_protocol::{
    constants::{self, v08_supported_adapter_runtime_proof as proof},
    AppGamePlatformProofStatusReadModel, AppGamePlatformProofStatusRow, LogFieldValue,
    APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_APPLICABLE, APP_GAME_PARENT_PLATFORM_ANDROID,
    APP_GAME_PARENT_PLATFORM_LINUX, APP_GAME_PARENT_PLATFORM_WINDOWS,
    APP_GAME_PLATFORM_GAP_ANDROID_DURABLE_USAGE_REPLAY,
    APP_GAME_PLATFORM_GAP_LINUX_FOREGROUND_CAPTURE,
    APP_GAME_PLATFORM_PROOF_SCOPED_WINDOWS_EXECUTION, APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL_ID,
};

use super::app_game_platform_proof_status_payload::{
    app_game_platform_proof_status_payload, app_game_platform_proof_status_read_model,
};

const GENERATED_AT: &str = "2026-06-08T16:50:00.000Z";

#[test]
fn platform_proof_status_payload_serializes_parent_safe_status_model() {
    let read_model = app_game_platform_proof_status_read_model(GENERATED_AT);
    let payload = app_game_platform_proof_status_payload(&read_model);

    let reparsed = serde_json::from_str::<AppGamePlatformProofStatusReadModel>(string_payload(
        &payload,
        constants::field::APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL,
    ))
    .expect("platform proof status read model reparses");

    assert_eq!(
        reparsed.read_model_id,
        APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL_ID
    );
    assert_eq!(reparsed.returned, 5);
    assert_eq!(reparsed.enforcement_ready_count, 1);
    assert!(!reparsed.platform_enforcement_claimed);
    assert_eq!(
        platform_row(&reparsed, APP_GAME_PARENT_PLATFORM_WINDOWS).proof_state,
        APP_GAME_PLATFORM_PROOF_SCOPED_WINDOWS_EXECUTION
    );
    assert!(reparsed
        .rows
        .iter()
        .any(|row| row.host_capability_state == APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_APPLICABLE));
    assert_refs(
        platform_row(&reparsed, APP_GAME_PARENT_PLATFORM_ANDROID),
        &[
            proof::REF_ANDROID_ADB_HOST_TOOLCHAIN,
            proof::REF_ANDROID_PHYSICAL_DEVICE_PROOF,
            proof::REF_ANDROID_USAGE_EVENTS_FOREGROUND,
        ],
        &[APP_GAME_PLATFORM_GAP_ANDROID_DURABLE_USAGE_REPLAY],
    );
    assert_refs(
        platform_row(&reparsed, APP_GAME_PARENT_PLATFORM_LINUX),
        &[
            proof::REF_LINUX_WSL_HOST_TOOLCHAIN,
            proof::REF_LINUX_WSLG_DISPLAY,
            proof::REF_LINUX_WSLG_X11_SOCKET,
            proof::REF_LINUX_WSLG_WAYLAND_SOCKET,
        ],
        &[APP_GAME_PLATFORM_GAP_LINUX_FOREGROUND_CAPTURE],
    );
}

fn string_payload<'a>(payload: &'a ocentra_parent_agent_protocol::LogFields, key: &str) -> &'a str {
    match payload.get(key) {
        Some(LogFieldValue::String(value)) => value.as_str(),
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn platform_row<'a>(
    read_model: &'a AppGamePlatformProofStatusReadModel,
    platform: &str,
) -> &'a AppGamePlatformProofStatusRow {
    read_model
        .rows
        .iter()
        .find(|row| row.platform == platform)
        .expect("platform row exists")
}

fn assert_refs(row: &AppGamePlatformProofStatusRow, proof_refs: &[&str], open_gaps: &[&str]) {
    for proof_ref in proof_refs {
        assert!(
            row.proof_refs.iter().any(|value| value == proof_ref),
            "missing proof ref {proof_ref}"
        );
    }
    for open_gap in open_gaps {
        assert!(
            row.open_gaps.iter().any(|value| value == open_gap),
            "missing open gap {open_gap}"
        );
    }
}
