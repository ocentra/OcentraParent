use super::local_ai_runtime_provider_proof::{
    LocalAiRuntimeProviderProofEntry, LocalAiRuntimeProviderProofReadModel,
    LocalAiRuntimeProviderProofRequirement, LocalAiRuntimeProviderProofStatus,
};
use super::{
    constants, DeviceRuntimeRole, LocalAiDegradedState, LocalAiProviderSchedulerJobClass,
    LocalAiProviderSchedulerLifecycle, LocalAiProviderSchedulerQueue,
    LocalAiProviderSchedulerStatus, LocalAiProviderSingletonScope, LocalAiResourceClass,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn local_ai_runtime_provider_proof_serializes_shared_parent_child_provider() {
    let read_model = LocalAiRuntimeProviderProofReadModel {
        schema_version: crate::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: constants::local_ai_runtime_provider_proof::READ_MODEL_ID.to_string(),
        generated_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        source_read_model_ids: vec!["local-ai-provider-scheduler".to_string()],
        entries: vec![proof_entry(
            ProofEntryIdCase::SharedParentChildProvider,
            LocalAiRuntimeProviderProofRequirement::SharedParentChildProvider,
            LocalAiRuntimeProviderProofStatus::Proved,
            ready_status(),
        )],
    };

    let serialized = serde_json::to_value(read_model).expect_value("proof read model serializes");

    assert_eq!(
        serialized["readModelId"],
        constants::local_ai_runtime_provider_proof::READ_MODEL_ID
    );
    assert_eq!(
        serialized["entries"][0]["requirement"],
        constants::local_ai_runtime_provider_proof::REQUIREMENT_SHARED_PARENT_CHILD_PROVIDER
    );
    assert_eq!(serialized["entries"][0]["runtimeLoadCount"], 1);
    assert_eq!(serialized["entries"][0]["runtimeAccessLaneCount"], 1);
    assert_eq!(serialized["entries"][0]["duplicateRuntimeBlocked"], true);
    assert_eq!(
        serialized["entries"][0]["participatingRoles"][2],
        constants::value::DEVICE_ROLE_AI_PROVIDER
    );
}

#[test]
fn local_ai_runtime_provider_proof_serializes_unavailable_reason() {
    let entry = proof_entry(
        ProofEntryIdCase::StatusContractHardening,
        LocalAiRuntimeProviderProofRequirement::ProviderStatusContractHardening,
        LocalAiRuntimeProviderProofStatus::Unavailable,
        unavailable_status(),
    );

    let serialized = serde_json::to_value(entry).expect_value("proof entry serializes");

    assert_eq!(
        serialized["proofStatus"],
        constants::local_ai_runtime_provider_proof::PROOF_STATUS_UNAVAILABLE
    );
    assert_eq!(
        serialized["unavailableReason"],
        constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED
    );
    assert_eq!(serialized["runtimeLoadCount"], 0);
    assert_eq!(serialized["runtimeAccessLaneCount"], 1);
}

#[derive(Clone, Copy)]
enum ProofEntryIdCase {
    SharedParentChildProvider,
    StatusContractHardening,
}

fn proof_entry(
    proof_entry_id: ProofEntryIdCase,
    requirement: LocalAiRuntimeProviderProofRequirement,
    proof_status: LocalAiRuntimeProviderProofStatus,
    source_scheduler_status: LocalAiProviderSchedulerStatus,
) -> LocalAiRuntimeProviderProofEntry {
    let runtime_load_count = if proof_status == LocalAiRuntimeProviderProofStatus::Unavailable {
        0
    } else {
        1
    };

    LocalAiRuntimeProviderProofEntry {
        schema_version: crate::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        proof_entry_id: match proof_entry_id {
            ProofEntryIdCase::SharedParentChildProvider => {
                constants::local_ai_runtime_provider_proof::ENTRY_ID_SHARED_PARENT_CHILD_PROVIDER
                    .to_string()
            }
            ProofEntryIdCase::StatusContractHardening => {
                constants::local_ai_runtime_provider_proof::ENTRY_ID_STATUS_CONTRACT_HARDENING
                    .to_string()
            }
        },
        requirement,
        proof_status,
        physical_device_id: source_scheduler_status.physical_device_id.clone(),
        singleton_scope: source_scheduler_status.singleton_scope,
        provider_id: source_scheduler_status.provider_id.clone(),
        runtime_reference_id: source_scheduler_status.runtime_reference_id.clone(),
        model_id: source_scheduler_status.model_id.clone(),
        model_reference: source_scheduler_status.model_reference.clone(),
        participating_roles: vec![
            DeviceRuntimeRole::ParentController,
            DeviceRuntimeRole::ChildAgent,
            DeviceRuntimeRole::AiProvider,
        ],
        accepted_job_classes: vec![
            LocalAiProviderSchedulerJobClass::ChildSafety,
            LocalAiProviderSchedulerJobClass::ParentAssistant,
        ],
        scheduler_lifecycle: source_scheduler_status.lifecycle_state,
        queue: source_scheduler_status.queue.clone(),
        degraded_state: source_scheduler_status.degraded_state,
        unavailable_reason: source_scheduler_status.unavailable_reason.clone(),
        source_scheduler_status,
        runtime_access_lane_count: 1,
        runtime_load_count,
        duplicate_runtime_blocked: runtime_load_count == 1,
        child_safety_priority_proved: false,
        parent_assistant_submission_allowed: runtime_load_count == 1,
        evidence_label: constants::local_ai_runtime_provider_proof::PROOF_SHARED_PARENT_CHILD
            .to_string(),
        capability_requirement:
            constants::local_ai_runtime_provider_proof::CAPABILITY_SHARED_PARENT_CHILD.to_string(),
        proof_requirement: constants::local_ai_runtime_provider_proof::PROOF_SHARED_PARENT_CHILD
            .to_string(),
        claim_boundary: constants::local_ai_runtime_provider_proof::CLAIM_SHARED_PROVIDER
            .to_string(),
        fallback_behavior: constants::local_ai_runtime_provider_proof::FALLBACK_QUEUE_OR_DEGRADE
            .to_string(),
        last_checked_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
    }
}

fn ready_status() -> LocalAiProviderSchedulerStatus {
    LocalAiProviderSchedulerStatus {
        physical_device_id: constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string(),
        singleton_scope: LocalAiProviderSingletonScope::PhysicalDevice,
        provider_id: constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI.to_string(),
        runtime_reference_id: constants::local_ai_runtime::RUNTIME_REFERENCE_LOCAL_LLAMA_CLI
            .to_string(),
        model_id: constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4.to_string(),
        model_reference: constants::local_ai_runtime::MODEL_REFERENCE_DEFAULT_GEMMA_4.to_string(),
        resource_class: LocalAiResourceClass::Cpu,
        lifecycle_state: LocalAiProviderSchedulerLifecycle::Running,
        current_job_class: Some(LocalAiProviderSchedulerJobClass::ParentAssistant),
        queue: LocalAiProviderSchedulerQueue::default(),
        duplicate_runtime_blocked: true,
        degraded_state: LocalAiDegradedState::None,
        unavailable_reason: None,
        last_checked_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
    }
}

fn unavailable_status() -> LocalAiProviderSchedulerStatus {
    let mut status = ready_status();
    status.provider_id = constants::local_ai_runtime::PROVIDER_ID_UNCONFIGURED.to_string();
    status.runtime_reference_id =
        constants::local_ai_runtime::RUNTIME_REFERENCE_DEV_UNCONFIGURED.to_string();
    status.model_id = constants::local_ai_runtime::MODEL_ID_UNCONFIGURED.to_string();
    status.model_reference = constants::local_ai_runtime::MODEL_REFERENCE_UNCONFIGURED.to_string();
    status.resource_class = LocalAiResourceClass::RemoteUnavailable;
    status.lifecycle_state = LocalAiProviderSchedulerLifecycle::Unavailable;
    status.current_job_class = None;
    status.duplicate_runtime_blocked = false;
    status.degraded_state = LocalAiDegradedState::ProviderUnavailable;
    status.unavailable_reason =
        Some(constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED.to_string());
    status
}
