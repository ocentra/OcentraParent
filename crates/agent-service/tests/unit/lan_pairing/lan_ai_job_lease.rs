use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::AgentCommandName;

use crate::{
    app::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test},
    lan_pairing_runtime_state::job_leases::LanAiJobLeaseTransition,
    lan_pairing_test_assertions::assert_rejection_with_audit,
    lan_pairing_test_commands::{
        command_for_target, intent_payload_for_kind, local_network_target, serialize_command,
    },
    test_require_ok::require_ok,
    test_require_some::require_some,
    test_text::TestText,
};

#[test]
fn service_owned_lan_ai_job_lease_claims_completes_and_reuses_completed_state() {
    let runtime = LanPairingRuntime::empty();
    let first = require_ok(
        runtime.claim_lan_ai_job_lease(&constants::lan_pairing::LAN_AI_JOB_ID),
        "initial LAN AI lease claim",
    );
    assert!(matches!(
        first,
        LanAiJobLeaseTransition::Claimed(state)
            if state.lease_state == constants::value::LAN_AI_LEASE_STATE_CLAIMED
                && state.attempt_count == 1
    ));

    let completed = require_some(
        runtime.complete_lan_ai_job_lease(&constants::lan_pairing::LAN_AI_JOB_ID),
        "completed LAN AI lease",
    );
    assert_eq!(
        completed.lease_state,
        constants::value::LAN_AI_LEASE_STATE_COMPLETED
    );

    let duplicate = require_ok(
        runtime.claim_lan_ai_job_lease(&constants::lan_pairing::LAN_AI_JOB_ID),
        "duplicate completed LAN AI lease",
    );
    assert!(matches!(
        duplicate,
        LanAiJobLeaseTransition::DuplicateCompleted(state)
            if state.lease_state == constants::value::LAN_AI_LEASE_STATE_COMPLETED
    ));
}

#[test]
fn service_owned_lan_ai_job_lease_rejects_active_duplicates() {
    let runtime = LanPairingRuntime::empty();
    let _first = require_ok(
        runtime.claim_lan_ai_job_lease(&constants::lan_pairing::LAN_AI_JOB_ID),
        "initial LAN AI lease claim",
    );
    let duplicate = require_ok(
        runtime.claim_lan_ai_job_lease(&constants::lan_pairing::LAN_AI_JOB_ID),
        "duplicate active LAN AI lease",
    );

    assert!(matches!(
        duplicate,
        LanAiJobLeaseTransition::DuplicateActiveRejected(state)
            if state.lease_state == constants::value::LAN_AI_LEASE_STATE_DUPLICATE_REJECTED
                && state.attempt_count == 1
    ));
}

#[test]
fn service_owned_lan_ai_job_lease_requeues_then_dead_letters_expired_work() {
    let runtime = LanPairingRuntime::empty();
    runtime.seed_lan_ai_job_lease_for_test(
        constants::lan_pairing::LAN_AI_JOB_ID,
        constants::value::LAN_AI_LEASE_STATE_CLAIMED,
        1,
        constants::lan_pairing::EXPIRED_AT,
    );
    let requeued = require_ok(
        runtime.claim_lan_ai_job_lease(&constants::lan_pairing::LAN_AI_JOB_ID),
        "expired LAN AI lease requeue",
    );
    assert!(matches!(
        requeued,
        LanAiJobLeaseTransition::ExpiredRequeued(state)
            if state.lease_state == constants::value::LAN_AI_LEASE_STATE_EXPIRED_REQUEUED
                && state.attempt_count == 2
    ));

    runtime.seed_lan_ai_job_lease_for_test(
        constants::lan_pairing::LAN_AI_JOB_ID,
        constants::value::LAN_AI_LEASE_STATE_CLAIMED,
        2,
        constants::lan_pairing::EXPIRED_AT,
    );
    let dead_lettered = require_ok(
        runtime.claim_lan_ai_job_lease(&constants::lan_pairing::LAN_AI_JOB_ID),
        "exhausted LAN AI lease",
    );
    assert!(matches!(
        dead_lettered,
        LanAiJobLeaseTransition::DeadLettered(state)
            if state.lease_state == constants::value::LAN_AI_LEASE_STATE_DEAD_LETTERED
                && state.attempt_count == 3
                && state.dead_letter_reason
                    == Some(constants::value::LAN_AI_DEAD_LETTER_REASON_MAX_ATTEMPTS)
    ));
}

#[tokio::test]
async fn unpaired_lan_ai_job_submit_rejects_before_allocating_a_service_lease() {
    let runtime = LanPairingRuntime::empty();
    let event = handle_command_text_for_test(
        serialize_command(command_for_target(
            AgentCommandName::AgentLanAiJobSubmit,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            lan_ai_job_payload(),
        )),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_rejection_with_audit(
        &event,
        constants::value::LAN_REASON_ANONYMOUS,
        constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_AI_LEASE_STATE),
        None
    );
    assert_eq!(
        event.payload.get(constants::field::LOCAL_AI_OUTPUT_TEXT),
        None
    );
}

fn lan_ai_job_payload() -> LogFields {
    let mut payload = intent_payload_for_kind(
        constants::lan_pairing::LAN_AI_JOB_INTENT_ID,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::PROOF_DIGEST,
        constants::lan_pairing::EXPIRES_AT,
        constants::value::LAN_INTENT_LAN_AI_JOB_SUBMIT,
    );
    payload.insert(
        constants::field::LAN_AI_JOB_ID.to_string(),
        LogFieldValue::String(constants::lan_pairing::LAN_AI_JOB_ID.to_string()),
    );
    payload.insert(
        constants::field::LOCAL_AI_CAPABILITY_FLAGS.to_string(),
        LogFieldValue::String(constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION.to_string()),
    );
    payload
}
