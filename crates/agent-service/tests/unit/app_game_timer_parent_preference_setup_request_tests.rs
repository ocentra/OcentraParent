#[path = "../support/test_invariants.rs"]
mod test_invariants;

use std::fs::{read_to_string, remove_file};
use std::path::PathBuf as TestPathBuf;
use std::primitive::str as TestStr;
use std::string::String as TestString;
use std::time::{SystemTime, UNIX_EPOCH};

use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel;
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED, APP_GAME_CONTROL_PERSISTENCE_REPLAYABLE,
};
use ocentra_parent_agent_protocol::app_game_timer_parent_preference_setup_request::{
    AppGameTimerParentPreferenceSetupRequest, AppGameTimerParentPreferenceSetupRequestResult,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_parent_agent_service::test_support::handle_local_command_text_for_test;

use crate::test_invariants::{
    require_json_decode, require_log_string_field, require_ok, require_some, serialize_test_json,
};
use crate::test_text::TestText;

use super::{
    app_game_child_runtime_transport_receipt_payload::app_game_child_runtime_transport_receipt_read_model_from_service_model,
    app_game_timer_parent_preference_setup_request::{
        build_activity_app_game_timer_parent_preference_setup_request_report_for_store_path,
        AppGameTimerSetupStorePath,
    },
};

const PERSISTED_SETUP_EVENT_COUNT: u64 = 14;

#[tokio::test]
async fn app_game_timer_parent_preference_setup_request_command_returns_accepted_boundary_result() {
    let body = serialize_test_json(&command_envelope());
    let event =
        handle_local_command_text_for_test(crate::test_text::TestText::from_display(body)).await;
    let result = request_payload(&crate::test_invariants::log_field(
        &event.payload,
        constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST,
        constants::error::AGENT_EVENT_SERIALIZES,
    ));

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityAppGameTimerParentPreferenceSetupRequested
    );
    assert_eq!(
        result.schema_version,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_REQUEST_SCHEMA_VERSION
    );
    assert_eq!(
        result.request_status,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_REQUEST_ACCEPTED
    );
    assert_eq!(
        result.action_result_reference_id,
        constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX
    );
    assert_eq!(
        result.action_result_reference_ids,
        vec![
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string()
        ]
    );
    assert!(result.command_boundary_claimed);
    assert!(result.action_result_handoff_claimed);
    assert!(
        result.action_result_persistence_status
            == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_PERSISTED
            || result.action_result_persistence_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_UNAVAILABLE
    );
    assert_eq!(
        result.parent_preference_mutation_receipt_id,
        setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_SUFFIX)
    );
    assert_eq!(
        result.parent_preference_mutation_receipt_ids,
        vec![
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_SUFFIX),
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string()
        ]
    );
    assert!(
        result.parent_preference_mutation_receipt_status
            == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_PERSISTED
            || result.parent_preference_mutation_receipt_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_UNAVAILABLE
    );
    assert_child_runtime_delivery_handoff_boundary(&result);
    assert_child_runtime_delivery_queue_boundary(&result);
    assert_child_runtime_delivery_dispatch_boundary(&result);
    assert_child_runtime_delivery_receipt_requirement_boundary(&result);
    assert_child_runtime_delivery_receipt_pending_boundary(&result);
    assert_child_runtime_delivery_receipt_ingested_boundary(&result);
    assert_durable_outbox_boundary(&result);
    assert_no_delivery_or_platform_claims(&result);
}

#[tokio::test]
async fn app_game_timer_parent_preference_setup_request_persists_action_result_row() {
    let store_path = temp_path(constants::activity_store::TEST_STORE_SUFFIX);
    cleanup_path(&store_path);

    let event =
        build_activity_app_game_timer_parent_preference_setup_request_report_for_store_path(
            command_envelope(),
            AppGameTimerSetupStorePath(store_path.clone()),
        )
        .await;
    let result = request_payload(&crate::test_invariants::log_field(
        &event.payload,
        constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST,
        constants::error::AGENT_EVENT_SERIALIZES,
    ));

    let store = require_ok(
        ActivityStore::open(&store_path),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    let model = require_ok(
        store.app_game_service_read_model(
            PERSISTED_SETUP_EVENT_COUNT,
            constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT,
        ),
        constants::error::ACTIVITY_STORE_QUERIES,
    );
    let status = require_ok(store.status(), constants::error::ACTIVITY_STORE_QUERIES);
    let outbox_path = store_path.with_extension(
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_FILE_EXTENSION,
    );
    let outbox_jsonl = require_ok(
        read_to_string(&outbox_path),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    cleanup_path(&store_path);

    assert_persisted_setup_result(&result);
    assert_eq!(status.events_stored, PERSISTED_SETUP_EVENT_COUNT);
    assert_persisted_action_result_model(&model);
    assert_persisted_setup_outbox(&result, &outbox_jsonl);
}

fn assert_persisted_setup_result(result: &AppGameTimerParentPreferenceSetupRequestResult) {
    assert_eq!(
        result.action_result_persistence_status,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_PERSISTED
    );
    assert!(result.action_result_persistence_claimed);
    assert_eq!(
        result.parent_preference_mutation_receipt_id,
        setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_SUFFIX)
    );
    assert_eq!(
        result.parent_preference_mutation_receipt_status,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_PERSISTED
    );
    assert!(result.parent_preference_mutation_receipt_claimed);
    assert_eq!(
        result.child_runtime_delivery_handoff_id,
        setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_SUFFIX)
    );
    assert_eq!(
        result.child_runtime_delivery_handoff_status,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_READY
    );
    assert!(result.child_runtime_delivery_handoff_claimed);
    assert_eq!(
        result.child_runtime_delivery_queue_id,
        setup_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_SUFFIX
        )
    );
    assert_eq!(
        result.child_runtime_delivery_queue_status,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_QUEUED
    );
    assert!(result.child_runtime_delivery_queue_claimed);
    assert_eq!(
        result.child_runtime_delivery_dispatch_id,
        setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_SUFFIX)
    );
    assert_eq!(
        result.child_runtime_delivery_dispatch_status,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_READY
    );
    assert!(result.child_runtime_delivery_dispatch_claimed);
    assert_eq!(
        result.child_runtime_delivery_receipt_requirement_id,
        setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIREMENT_SUFFIX)
    );
    assert_eq!(
        result.child_runtime_delivery_receipt_requirement_status,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIRED
    );
    assert!(result.child_runtime_delivery_receipt_requirement_claimed);
    assert_eq!(
        result.child_runtime_delivery_receipt_pending_id,
        setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_PENDING_SUFFIX)
    );
    assert_eq!(
        result.child_runtime_delivery_receipt_pending_status,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_PENDING
    );
    assert!(result.child_runtime_delivery_receipt_pending_claimed);
    assert_eq!(
        result.child_runtime_delivery_receipt_ingested_id,
        setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_INGESTED_SUFFIX)
    );
    assert_eq!(
        result.child_runtime_delivery_receipt_ingested_status,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_PENDING
    );
    assert!(!result.child_runtime_delivery_receipt_ingested_claimed);
    assert_no_delivery_or_platform_claims(result);
}

fn assert_persisted_action_result_model(model: &AppGameServiceReadModel) {
    assert_eq!(model.approval_action_result_returned, 1);
    assert_eq!(
        model.approval_action_result_rows[0].result_id,
        constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX
    );
    assert_eq!(
        model.approval_action_result_rows[0].result_status,
        APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED
    );
    assert_eq!(
        model.approval_action_result_rows[0]
            .decision
            .persistence_state,
        APP_GAME_CONTROL_PERSISTENCE_REPLAYABLE
    );
    assert!(model.approval_action_result_rows[0]
        .enforcement_result
        .is_none());

    let receipt_model =
        app_game_child_runtime_transport_receipt_read_model_from_service_model(model.clone());

    assert_eq!(receipt_model.returned, 1);
    assert_eq!(receipt_model.manual_required_count, 1);
    assert_eq!(
        receipt_model.rows[0].source_runtime_writer_row_id,
        constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX
    );
    assert_eq!(
        receipt_model.rows[0].required_transport_refs,
        vec![
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string(),
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
            ocentra_parent_agent_protocol::app_game_child_runtime_transport_receipt::APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_TRANSPORT_NOT_EXECUTED.to_string()
        ]
    );
    assert!(!receipt_model.runtime_transport_executed);
    assert!(!receipt_model.runtime_receipt_ingested);
}

fn assert_child_runtime_delivery_handoff_boundary(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) {
    assert_eq!(
        result.child_runtime_delivery_handoff_id,
        setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_SUFFIX)
    );
    assert_eq!(
        result.child_runtime_delivery_handoff_ids,
        vec![
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_SUFFIX),
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_SUFFIX),
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string(),
        ]
    );
    assert!(
        result.child_runtime_delivery_handoff_status
            == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_READY
            || result.child_runtime_delivery_handoff_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_UNAVAILABLE
    );
}

fn assert_child_runtime_delivery_queue_boundary(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) {
    assert_eq!(
        result.child_runtime_delivery_queue_id,
        setup_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_SUFFIX
        )
    );
    assert_eq!(
        result.child_runtime_delivery_queue_ids,
        vec![
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_SUFFIX),
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_SUFFIX),
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_SUFFIX),
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string(),
        ]
    );
    assert!(
        result.child_runtime_delivery_queue_status
            == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_QUEUED
            || result.child_runtime_delivery_queue_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_UNAVAILABLE
    );
}

fn assert_child_runtime_delivery_dispatch_boundary(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) {
    assert_eq!(
        result.child_runtime_delivery_dispatch_id,
        setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_SUFFIX)
    );
    assert_eq!(
        result.child_runtime_delivery_dispatch_ids,
        vec![
            setup_id(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_SUFFIX
            ),
            setup_id(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_SUFFIX
            ),
            setup_id(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_SUFFIX
            ),
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_SUFFIX),
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string(),
        ]
    );
    assert!(
        result.child_runtime_delivery_dispatch_status
            == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_READY
            || result.child_runtime_delivery_dispatch_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_UNAVAILABLE
    );
}

fn assert_child_runtime_delivery_receipt_requirement_boundary(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) {
    assert_eq!(
        result.child_runtime_delivery_receipt_requirement_id,
        setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIREMENT_SUFFIX)
    );
    assert_eq!(
        result.child_runtime_delivery_receipt_requirement_ids,
        vec![
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIREMENT_SUFFIX),
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_SUFFIX),
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_SUFFIX),
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_SUFFIX),
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_SUFFIX),
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string(),
        ]
    );
    assert!(
        result.child_runtime_delivery_receipt_requirement_status
            == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIRED
            || result.child_runtime_delivery_receipt_requirement_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_UNAVAILABLE
    );
}

fn assert_child_runtime_delivery_receipt_pending_boundary(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) {
    assert_eq!(
        result.child_runtime_delivery_receipt_pending_id,
        setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_PENDING_SUFFIX)
    );
    assert_eq!(
        result.child_runtime_delivery_receipt_pending_ids,
        vec![
            setup_id(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_PENDING_SUFFIX
            ),
            setup_id(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIREMENT_SUFFIX
            ),
            setup_id(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_SUFFIX
            ),
            setup_id(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_SUFFIX
            ),
            setup_id(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_SUFFIX
            ),
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_SUFFIX),
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string(),
        ]
    );
    assert!(
        result.child_runtime_delivery_receipt_pending_status
            == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_PENDING
            || result.child_runtime_delivery_receipt_pending_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_UNAVAILABLE
    );
}

fn assert_child_runtime_delivery_receipt_ingested_boundary(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) {
    assert_eq!(
        result.child_runtime_delivery_receipt_ingested_id,
        setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_INGESTED_SUFFIX)
    );
    assert_eq!(
        result.child_runtime_delivery_receipt_ingested_ids,
        vec![
            setup_id(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_INGESTED_SUFFIX
            ),
            setup_id(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_PENDING_SUFFIX
            ),
            setup_id(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIREMENT_SUFFIX
            ),
            setup_id(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_SUFFIX
            ),
            setup_id(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_SUFFIX
            ),
            setup_id(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_SUFFIX
            ),
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_SUFFIX),
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string(),
        ]
    );
    assert!(
        result.child_runtime_delivery_receipt_ingested_status
            == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_PENDING
            || result.child_runtime_delivery_receipt_ingested_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_UNAVAILABLE
    );
    assert!(!result.child_runtime_delivery_receipt_ingested_claimed);
}

macro_rules! assert_provider_delivery_requirement_boundary {
    ($result:expr) => {
        assert_eq!(
            $result.provider_delivery_adapter_requirement_id,
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_ADAPTER_REQUIREMENT_SUFFIX)
        );
        assert_eq!(
            $result.provider_delivery_adapter_requirement_ids,
            vec![
                setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_ADAPTER_REQUIREMENT_SUFFIX),
                setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_ATTEMPT_SUFFIX),
            ]
        );
        assert!(
            $result.provider_delivery_adapter_requirement_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_ADAPTER_REQUIRED
                || $result.provider_delivery_adapter_requirement_status
                    == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_UNAVAILABLE
        );
        assert_eq!(
            $result.provider_delivery_credential_requirement_id,
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_CREDENTIAL_REQUIREMENT_SUFFIX)
        );
        assert_eq!(
            $result.provider_delivery_credential_requirement_ids,
            vec![
                setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_CREDENTIAL_REQUIREMENT_SUFFIX),
                setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_ADAPTER_REQUIREMENT_SUFFIX),
            ]
        );
        assert!(
            $result.provider_delivery_credential_requirement_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_CREDENTIAL_PROOF_REQUIRED
                || $result.provider_delivery_credential_requirement_status
                    == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_UNAVAILABLE
        );
        assert_eq!(
            $result.provider_delivery_queue_id,
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_QUEUE_SUFFIX)
        );
        assert_eq!(
            $result.provider_delivery_queue_ids,
            vec![
                setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_QUEUE_SUFFIX),
                setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_CREDENTIAL_REQUIREMENT_SUFFIX),
            ]
        );
        assert!(
            $result.provider_delivery_queue_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_QUEUE_QUEUED
                || $result.provider_delivery_queue_status
                    == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_UNAVAILABLE
        );
        assert_eq!(
            $result.provider_delivery_receipt_requirement_id,
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_REQUIREMENT_SUFFIX)
        );
        assert_eq!(
            $result.provider_delivery_receipt_requirement_ids,
            vec![
                setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_REQUIREMENT_SUFFIX),
                setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_QUEUE_SUFFIX),
            ]
        );
        assert!(
            $result.provider_delivery_receipt_requirement_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_REQUIRED
                || $result.provider_delivery_receipt_requirement_status
                    == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_UNAVAILABLE
        );
        assert_eq!(
            $result.provider_delivery_receipt_pending_id,
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_PENDING_SUFFIX)
        );
        assert_eq!(
            $result.provider_delivery_receipt_pending_ids,
            vec![
                setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_PENDING_SUFFIX),
                setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_REQUIREMENT_SUFFIX),
            ]
        );
        assert!(
            $result.provider_delivery_receipt_pending_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_PENDING
                || $result.provider_delivery_receipt_pending_status
                    == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_UNAVAILABLE
        );
        assert_eq!(
            $result.provider_delivery_receipt_ingested_id,
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_INGESTED_SUFFIX)
        );
        assert_eq!(
            $result.provider_delivery_receipt_ingested_ids,
            vec![
                setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_INGESTED_SUFFIX),
                setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_PENDING_SUFFIX),
            ]
        );
        assert!(
            $result.provider_delivery_receipt_ingested_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_PENDING
                || $result.provider_delivery_receipt_ingested_status
                    == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_UNAVAILABLE
        );
        assert!(!$result.provider_delivery_receipt_ingested_claimed);
    };
}

macro_rules! assert_outbox_provider_preflight_requirements {
    ($outbox_record:expr, $result:expr) => {
        assert_eq!(
            $outbox_record
                [constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_ADAPTER_REQUIREMENT_ID],
            $result.provider_delivery_adapter_requirement_id
        );
        assert_eq!(
            $outbox_record
                [constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_ADAPTER_REQUIREMENT_STATUS],
            $result.provider_delivery_adapter_requirement_status
        );
        assert_eq!(
            $outbox_record
                [constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_CREDENTIAL_REQUIREMENT_ID],
            $result.provider_delivery_credential_requirement_id
        );
        assert_eq!(
            $outbox_record
                [constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_CREDENTIAL_REQUIREMENT_STATUS],
            $result.provider_delivery_credential_requirement_status
        );
        assert_eq!(
            $outbox_record
                [constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_QUEUE_ID],
            $result.provider_delivery_queue_id
        );
        assert_eq!(
            $outbox_record
                [constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_QUEUE_STATUS],
            $result.provider_delivery_queue_status
        );
        assert_eq!(
            $outbox_record
                [constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_RECEIPT_REQUIREMENT_ID],
            $result.provider_delivery_receipt_requirement_id
        );
        assert_eq!(
            $outbox_record
                [constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_RECEIPT_REQUIREMENT_STATUS],
            $result.provider_delivery_receipt_requirement_status
        );
        assert_eq!(
            $outbox_record
                [constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_RECEIPT_PENDING_ID],
            $result.provider_delivery_receipt_pending_id
        );
        assert_eq!(
            $outbox_record
                [constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_RECEIPT_PENDING_STATUS],
            $result.provider_delivery_receipt_pending_status
        );
        assert_eq!(
            $outbox_record
                [constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_RECEIPT_INGESTED_ID],
            $result.provider_delivery_receipt_ingested_id
        );
        assert_eq!(
            $outbox_record
                [constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_RECEIPT_INGESTED_STATUS],
            $result.provider_delivery_receipt_ingested_status
        );
    };
}

macro_rules! assert_provider_delivery_persisted_statuses {
    ($result:expr) => {
        assert_eq!(
            $result.provider_delivery_readiness_status,
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_MANUAL_REQUIRED
        );
        assert!($result.provider_delivery_readiness_claimed);
        assert_eq!(
            $result.provider_delivery_attempt_status,
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_ATTEMPT_MANUAL_REQUIRED
        );
        assert!($result.provider_delivery_attempt_claimed);
        assert_eq!(
            $result.provider_delivery_adapter_requirement_status,
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_ADAPTER_REQUIRED
        );
        assert!($result.provider_delivery_adapter_requirement_claimed);
        assert_eq!(
            $result.provider_delivery_credential_requirement_status,
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_CREDENTIAL_PROOF_REQUIRED
        );
        assert!($result.provider_delivery_credential_requirement_claimed);
        assert_eq!(
            $result.provider_delivery_queue_status,
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_QUEUE_QUEUED
        );
        assert!($result.provider_delivery_queue_claimed);
        assert_eq!(
            $result.provider_delivery_receipt_requirement_status,
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_REQUIRED
        );
        assert!($result.provider_delivery_receipt_requirement_claimed);
        assert_eq!(
            $result.provider_delivery_receipt_pending_status,
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_PENDING
        );
        assert!($result.provider_delivery_receipt_pending_claimed);
        assert_eq!(
            $result.provider_delivery_receipt_ingested_status,
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_PENDING
        );
        assert!(!$result.provider_delivery_receipt_ingested_claimed);
    };
}

fn assert_durable_outbox_boundary(result: &AppGameTimerParentPreferenceSetupRequestResult) {
    assert_eq!(
        result.durable_outbox_record_id,
        setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_SUFFIX)
    );
    assert_eq!(
        result.durable_outbox_record_ids[0],
        setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_SUFFIX)
    );
    assert!(
        result.durable_outbox_status
            == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_RECORDED
            || result.durable_outbox_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_UNAVAILABLE
    );
    assert_eq!(
        result.provider_delivery_readiness_id,
        setup_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_READINESS_SUFFIX
        )
    );
    assert_eq!(
        result.provider_delivery_readiness_ids,
        vec![
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_READINESS_SUFFIX),
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_SUFFIX),
        ]
    );
    assert!(
        result.provider_delivery_readiness_status
            == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_MANUAL_REQUIRED
            || result.provider_delivery_readiness_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_UNAVAILABLE
    );
    assert_eq!(
        result.provider_delivery_attempt_id,
        setup_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_ATTEMPT_SUFFIX
        )
    );
    assert_eq!(
        result.provider_delivery_attempt_ids,
        vec![
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_ATTEMPT_SUFFIX),
            setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_READINESS_SUFFIX),
        ]
    );
    assert!(
        result.provider_delivery_attempt_status
            == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_ATTEMPT_MANUAL_REQUIRED
            || result.provider_delivery_attempt_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_UNAVAILABLE
    );
    assert_provider_delivery_requirement_boundary!(result);
}

fn assert_persisted_setup_outbox(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
    outbox_jsonl: &TestStr,
) {
    assert_eq!(
        result.durable_outbox_record_id,
        setup_id(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_SUFFIX)
    );
    assert_eq!(
        result.durable_outbox_status,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_RECORDED
    );
    assert!(result.durable_outbox_claimed);
    assert_provider_delivery_persisted_statuses!(result);
    let first_line = require_some(
        outbox_jsonl.lines().next(),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    let outbox_record: serde_json::Value =
        require_json_decode(first_line, constants::error::AGENT_EVENT_SERIALIZES);
    assert_eq!(
        outbox_record[constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_RECORD_ID],
        result.durable_outbox_record_id
    );
    assert_eq!(
        outbox_record[constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_REQUEST_ID],
        result.request_id
    );
    assert_eq!(
        outbox_record
            [constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_CHILD_RUNTIME_DELIVERY_RECEIPT_INGESTED_ID],
        result.child_runtime_delivery_receipt_ingested_id
    );
    assert_outbox_provider_preflight_requirements!(outbox_record, result);
    assert_eq!(
        outbox_record
            [constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_CLAIMED],
        false
    );
    assert_eq!(
        outbox_record
            [constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_RECEIPT_INGESTION_CLAIMED],
        false
    );
    assert_eq!(
        outbox_record
            [constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_ADAPTER_DISPATCH_CLAIMED],
        false
    );
    assert_eq!(
        outbox_record
            [constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PLATFORM_ENFORCEMENT_CLAIMED],
        false
    );
}

fn assert_no_delivery_or_platform_claims(result: &AppGameTimerParentPreferenceSetupRequestResult) {
    assert!(!result.parent_preference_mutation_claimed);
    assert!(!result.notification_rule_mutation_claimed);
    assert!(!result.provider_delivery_claimed);
    assert!(!result.provider_receipt_ingestion_claimed);
    assert!(!result.child_runtime_delivery_claimed);
    assert!(!result.adapter_dispatch_claimed);
    assert!(!result.broad_blocking_claimed);
    assert!(!result.platform_enforcement_claimed);
    assert!(!result.raw_private_source_rows_claimed);
    assert!(!result.raw_target_values_claimed);
    assert!(!result.private_diagnostics_claimed);
}

fn setup_id(suffix: &TestStr) -> TestString {
    let mut setup_id =
        constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string();
    setup_id.push(constants::delimiter::HYPHEN);
    setup_id.push_str(suffix);
    setup_id
}

fn command_envelope() -> AgentCommandEnvelope {
    let request = AppGameTimerParentPreferenceSetupRequest {
        request_id: constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUESTED
            .to_string(),
        requested_at: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT
            .to_string(),
        parent_surface_intent_reference_id:
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string(),
        parent_preference_setup_reference_id:
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
        request_reference_ids: vec![
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string(),
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
        ],
    };
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST.to_string(),
        LogFieldValue::String(serialize_test_json(&request)),
    );
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUESTED
            .to_string(),
        sent_at: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform:
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
                    .to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentActivityAppGameTimerParentPreferenceSetupRequest,
        payload,
    }
}

fn request_payload(value: &LogFieldValue) -> AppGameTimerParentPreferenceSetupRequestResult {
    let text = require_log_string_field(Some(value), constants::error::AGENT_EVENT_SERIALIZES);
    require_json_decode(text, constants::error::AGENT_EVENT_SERIALIZES)
}

fn temp_path(suffix: TestText) -> TestPathBuf {
    let suffix = suffix;
    let mut name = TestString::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix.as_ref());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST);
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&unique_suffix());

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::activity_store::FILE_EXTENSION);
    path
}

fn unique_suffix() -> TestString {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos().to_string())
        .unwrap_or_else(|_| TestString::from("0"))
}

fn cleanup_path(path: &TestPathBuf) {
    let _ = remove_file(path);
    let mut wal_path = path.clone();
    wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let mut shm_path = path.clone();
    shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
    let _ = remove_file(path.with_extension(
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_FILE_EXTENSION,
    ));
}
