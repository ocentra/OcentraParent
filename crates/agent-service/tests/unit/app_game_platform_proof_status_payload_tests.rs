use ocentra_parent_agent_protocol::app_game_adapter_execution_readiness::APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_APPLICABLE;
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    APP_GAME_PARENT_PLATFORM_ANDROID, APP_GAME_PARENT_PLATFORM_LINUX,
    APP_GAME_PARENT_PLATFORM_WINDOWS,
};
use ocentra_parent_agent_protocol::app_game_platform_proof_status::{
    APP_GAME_PLATFORM_GAP_ANDROID_DURABLE_USAGE_REPLAY,
    APP_GAME_PLATFORM_GAP_LINUX_FOREGROUND_CAPTURE,
    APP_GAME_PLATFORM_PROOF_SCOPED_WINDOWS_EXECUTION, APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL_ID,
};
use ocentra_parent_agent_protocol::constants::{
    self, v08_supported_adapter_runtime_proof as proof,
};
use ocentra_parent_agent_protocol::AppGamePlatformProofStatusReadModel;
use ocentra_parent_agent_protocol::AppGamePlatformProofStatusRow;
use std::primitive::str as TestStr;

use crate::test_invariants::{require_json_decode, require_log_string_field, require_some};

use super::app_game_platform_proof_status_payload::{
    app_game_platform_proof_status_payload, app_game_platform_proof_status_read_model,
};

const GENERATED_AT: &TestStr = constants::value::APP_GAME_TEST_PLATFORM_PROOF_STATUS_GENERATED_AT;

#[test]
fn platform_proof_status_payload_serializes_parent_safe_status_model() {
    let read_model = app_game_platform_proof_status_read_model(
        super::app_game_adapter_execution_readiness_payload::GeneratedAtText(
            GENERATED_AT.to_string(),
        ),
    );
    let payload = app_game_platform_proof_status_payload(&read_model);

    let reparsed = require_json_decode::<AppGamePlatformProofStatusReadModel>(
        string_payload(
            &payload,
            constants::field::APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL,
        ),
        constants::value::APP_GAME_TEST_PLATFORM_PROOF_STATUS_REPARSES,
    );

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

fn string_payload<'a>(
    payload: &'a ocentra_parent_agent_protocol::logging::LogFields,
    field_name: &TestStr,
) -> &'a TestStr {
    require_log_string_field(
        payload.get(field_name),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
}

fn platform_row<'a>(
    read_model: &'a AppGamePlatformProofStatusReadModel,
    platform: &TestStr,
) -> &'a AppGamePlatformProofStatusRow {
    require_some(
        read_model.rows.iter().find(|row| row.platform == platform),
        constants::value::APP_GAME_TEST_PLATFORM_PROOF_STATUS_ROW_EXISTS,
    )
}

fn assert_refs(
    row: &AppGamePlatformProofStatusRow,
    proof_refs: &[&TestStr],
    open_gaps: &[&TestStr],
) {
    for proof_ref in proof_refs {
        assert!(row.proof_refs.iter().any(|value| value == proof_ref));
    }
    for open_gap in open_gaps {
        assert!(row.open_gaps.iter().any(|value| value == open_gap));
    }
}
