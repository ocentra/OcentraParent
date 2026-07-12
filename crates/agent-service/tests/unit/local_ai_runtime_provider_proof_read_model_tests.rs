use std::primitive::str as TestStr;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeRole;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiDegradedState;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiResourceClass;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobClass;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerLifecycle;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;
use ocentra_parent_agent_protocol::local_ai_runtime_provider_proof::{
    LocalAiRuntimeProviderProofEntry, LocalAiRuntimeProviderProofReadModel,
};

use crate::{
    local_ai_provider_scheduler::LocalAiProviderSchedulerRuntime,
    local_ai_provider_scheduler_state::LocalAiPhysicalDeviceId,
    local_ai_runtime_provider_proof_read_model::local_ai_runtime_provider_proof_read_model,
    test_invariants::{require_ok, require_some},
};

#[test]
fn local_ai_runtime_provider_proof_read_model_captures_all_requirements() {
    let scheduler = LocalAiProviderSchedulerRuntime::new();
    scheduler.record_running_job_for_device(
        LocalAiPhysicalDeviceId(constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string()),
        &ready_runtime(),
        LocalAiProviderSchedulerJobClass::ParentAssistant,
    );

    let read_model = local_ai_runtime_provider_proof_read_model(
        constants::local_ai_runtime::TEST_CHECKED_AT,
        &scheduler.status_snapshot(),
    );

    assert_eq!(read_model.entries.len(), 8);
    assert_eq!(
        read_model.read_model_id,
        constants::local_ai_runtime_provider_proof::READ_MODEL_ID
    );
    assert!(read_model
        .entries
        .iter()
        .all(|entry| entry.runtime_load_count <= 1));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| entry.runtime_access_lane_count == 1));
    assert!(read_model.entries.iter().any(|entry| {
        entry.proof_entry_id
            == constants::local_ai_runtime_provider_proof::ENTRY_ID_SHARED_PARENT_CHILD_PROVIDER
            && entry.participating_roles == shared_roles()
    }));
}

#[test]
fn local_ai_runtime_provider_proof_keeps_child_safety_priority_and_duplicate_blocking() {
    let scheduler = LocalAiProviderSchedulerRuntime::new();
    scheduler.record_running_job_for_device(
        LocalAiPhysicalDeviceId(constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string()),
        &ready_runtime(),
        LocalAiProviderSchedulerJobClass::ParentAssistant,
    );

    let read_model = local_ai_runtime_provider_proof_read_model(
        constants::local_ai_runtime::TEST_CHECKED_AT,
        &scheduler.status_snapshot(),
    );
    let priority = entry(
        &read_model,
        constants::local_ai_runtime_provider_proof::ENTRY_ID_CHILD_SAFETY_PRIORITY,
    );
    let no_duplicate = entry(
        &read_model,
        constants::local_ai_runtime_provider_proof::ENTRY_ID_NO_DUPLICATE_MODEL_LOAD,
    );

    assert_eq!(
        priority.scheduler_lifecycle,
        LocalAiProviderSchedulerLifecycle::Queued
    );
    assert_eq!(
        priority.source_scheduler_status.current_job_class,
        Some(LocalAiProviderSchedulerJobClass::ParentReport)
    );
    assert_eq!(priority.queue.child_safety_queued, 1);
    assert_eq!(priority.queue.parent_assistant_queued, 1);
    assert!(priority.child_safety_priority_proved);
    assert!(no_duplicate.duplicate_runtime_blocked);
    assert_eq!(no_duplicate.runtime_access_lane_count, 1);
    assert_eq!(no_duplicate.runtime_load_count, 1);
}

#[test]
fn local_ai_runtime_provider_proof_preserves_degraded_and_unavailable_boundaries() {
    let scheduler = LocalAiProviderSchedulerRuntime::new();
    scheduler.record_running_job_for_device(
        LocalAiPhysicalDeviceId(constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string()),
        &ready_runtime(),
        LocalAiProviderSchedulerJobClass::ParentAssistant,
    );

    let read_model = local_ai_runtime_provider_proof_read_model(
        constants::local_ai_runtime::TEST_CHECKED_AT,
        &scheduler.status_snapshot(),
    );
    let degraded = entry(
        &read_model,
        constants::local_ai_runtime_provider_proof::ENTRY_ID_QUEUED_DEGRADED_LIFECYCLE,
    );
    let unavailable = entry(
        &read_model,
        constants::local_ai_runtime_provider_proof::ENTRY_ID_STATUS_CONTRACT_HARDENING,
    );

    assert_eq!(
        degraded.scheduler_lifecycle,
        LocalAiProviderSchedulerLifecycle::Degraded
    );
    assert_eq!(degraded.degraded_state, LocalAiDegradedState::Overloaded);
    assert!(degraded.parent_assistant_submission_allowed);
    assert_eq!(
        unavailable.scheduler_lifecycle,
        LocalAiProviderSchedulerLifecycle::Unavailable
    );
    assert_eq!(unavailable.runtime_load_count, 0);
    assert_eq!(
        unavailable.unavailable_reason.as_deref(),
        Some(constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED)
    );
}

#[test]
fn local_ai_runtime_provider_proof_serializes_for_protocol_parity() {
    let scheduler = LocalAiProviderSchedulerRuntime::new();
    scheduler.record_running_job_for_device(
        LocalAiPhysicalDeviceId(constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string()),
        &ready_runtime(),
        LocalAiProviderSchedulerJobClass::ParentAssistant,
    );
    let read_model = local_ai_runtime_provider_proof_read_model(
        constants::local_ai_runtime::TEST_CHECKED_AT,
        &scheduler.status_snapshot(),
    );

    let serialized = require_ok(
        serde_json::to_value(read_model),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert_eq!(
        serialized[constants::field::ENTRIES][3]
            [constants::field::LOCAL_AI_PROVIDER_PROOF_REQUIREMENT],
        constants::local_ai_runtime_provider_proof::REQUIREMENT_CHILD_SAFETY_PRIORITY
    );
    assert_eq!(
        serialized[constants::field::ENTRIES][3]
            [constants::field::LOCAL_AI_PROVIDER_PROOF_CHILD_PRIORITY_PROVED],
        true
    );
    assert_eq!(
        serialized[constants::field::ENTRIES][5]
            [constants::field::LOCAL_AI_PROVIDER_PROOF_ACCEPTED_JOB_CLASSES][0],
        constants::local_ai_runtime::SCHEDULER_JOB_PARENT_ASSISTANT
    );
}

fn entry<'a>(
    read_model: &'a LocalAiRuntimeProviderProofReadModel,
    proof_entry_id: &TestStr,
) -> &'a LocalAiRuntimeProviderProofEntry {
    require_some(
        read_model
            .entries
            .iter()
            .find(|candidate| candidate.proof_entry_id == proof_entry_id),
        constants::local_ai_runtime_provider_proof::READ_MODEL_ID,
    )
}

fn shared_roles() -> Vec<DeviceRuntimeRole> {
    vec![
        DeviceRuntimeRole::ParentController,
        DeviceRuntimeRole::ChildAgent,
        DeviceRuntimeRole::AiProvider,
    ]
}

fn ready_runtime() -> LocalModelRuntimeStatus {
    LocalModelRuntimeStatus {
        runtime_reference_id: constants::local_ai_runtime::RUNTIME_REFERENCE_LOCAL_LLAMA_CLI
            .to_string(),
        provider_id: constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI.to_string(),
        model_id: constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4.to_string(),
        model_reference: constants::local_ai_runtime::MODEL_REFERENCE_DEFAULT_GEMMA_4.to_string(),
        privacy_mode: ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiProviderPrivacyMode::LocalOnly,
        adapter_boundary: ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiAdapterBoundary::LocalAdapterReady,
        execution_state: ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiExecutionState::DryRunReady,
        provider_source: ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiProviderSource::LocalModelCache,
        load_state: ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiModelLoadState::Loaded,
        capability_flags: vec![
            ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiCapabilityFlag::ChatCompletion,
        ],
        resource_class: LocalAiResourceClass::Cpu,
        degraded_state: LocalAiDegradedState::None,
        last_checked_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        unavailable_reason: None,
    }
}
