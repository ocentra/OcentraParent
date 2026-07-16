use ocentra_parent_agent_protocol::constants::{self, local_ai_runtime_provider_proof as proof};
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeRole;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiDegradedState;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiResourceClass;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobClass;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerLifecycle;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerQueue;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerStatus;
use ocentra_parent_agent_protocol::local_ai_runtime_provider_proof::{
    LocalAiRuntimeProviderProofEntry, LocalAiRuntimeProviderProofReadModel,
    LocalAiRuntimeProviderProofRequirement, LocalAiRuntimeProviderProofStatus,
};
use ocentra_parent_agent_protocol::policy_constants;

use crate::local_ai_runtime_config_values::LocalAiRuntimeText;

pub(crate) fn local_ai_runtime_provider_proof_read_model(
    generated_at: impl Into<LocalAiRuntimeText>,
    scheduler_status: &LocalAiProviderSchedulerStatus,
) -> LocalAiRuntimeProviderProofReadModel {
    let generated_at = generated_at.into();
    let queued_status = queued_priority_status(scheduler_status);
    let degraded_status = degraded_provider_status(scheduler_status);
    let unavailable_status = unavailable_provider_status(&generated_at);

    LocalAiRuntimeProviderProofReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: proof::READ_MODEL_ID.to_string(),
        generated_at: generated_at.0,
        source_read_model_ids: vec![
            proof::SOURCE_LOCAL_AI_PROVIDER_SCHEDULER.to_string(),
            proof::SOURCE_DEVICE_ROLE_RUNTIME_READ_MODEL.to_string(),
            proof::SOURCE_PARENT_ASSISTANT_RUNTIME.to_string(),
        ],
        entries: vec![
            entry(entry_spec_one_provider_role(), scheduler_status, 1),
            entry(entry_spec_shared_parent_child(), scheduler_status, 1),
            entry(entry_spec_single_runtime_lane(), &queued_status, 1),
            entry(entry_spec_child_safety_priority(), &queued_status, 1),
            entry(entry_spec_degraded_lifecycle(), &degraded_status, 1),
            entry(entry_spec_parent_assistant_submit(), scheduler_status, 1),
            entry(entry_spec_no_duplicate_load(), &queued_status, 1),
            entry(entry_spec_status_hardening(), &unavailable_status, 0),
        ],
    }
}

struct EntrySpec<'a> {
    proof_entry_id: &'a str,
    requirement: LocalAiRuntimeProviderProofRequirement,
    proof_status: LocalAiRuntimeProviderProofStatus,
    participating_roles: Vec<DeviceRuntimeRole>,
    accepted_job_classes: Vec<LocalAiProviderSchedulerJobClass>,
    child_safety_priority_proved: bool,
    parent_assistant_submission_allowed: bool,
    evidence_label: &'a str,
    capability_requirement: &'a str,
    proof_requirement: &'a str,
    claim_boundary: &'a str,
    fallback_behavior: &'a str,
}

fn entry(
    spec: EntrySpec<'_>,
    source_status: &LocalAiProviderSchedulerStatus,
    runtime_load_count: u16,
) -> LocalAiRuntimeProviderProofEntry {
    LocalAiRuntimeProviderProofEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        proof_entry_id: spec.proof_entry_id.to_string(),
        requirement: spec.requirement,
        proof_status: spec.proof_status,
        physical_device_id: source_status.physical_device_id.clone(),
        singleton_scope: source_status.singleton_scope,
        provider_id: source_status.provider_id.clone(),
        runtime_reference_id: source_status.runtime_reference_id.clone(),
        model_id: source_status.model_id.clone(),
        model_reference: source_status.model_reference.clone(),
        participating_roles: spec.participating_roles,
        accepted_job_classes: spec.accepted_job_classes,
        scheduler_lifecycle: source_status.lifecycle_state,
        source_scheduler_status: source_status.clone(),
        runtime_access_lane_count: 1,
        runtime_load_count,
        duplicate_runtime_blocked: source_status.duplicate_runtime_blocked,
        child_safety_priority_proved: spec.child_safety_priority_proved,
        parent_assistant_submission_allowed: spec.parent_assistant_submission_allowed,
        queue: source_status.queue.clone(),
        degraded_state: source_status.degraded_state,
        unavailable_reason: source_status.unavailable_reason.clone(),
        evidence_label: spec.evidence_label.to_string(),
        capability_requirement: spec.capability_requirement.to_string(),
        proof_requirement: spec.proof_requirement.to_string(),
        claim_boundary: spec.claim_boundary.to_string(),
        fallback_behavior: spec.fallback_behavior.to_string(),
        last_checked_at: source_status.last_checked_at.clone(),
    }
}

fn entry_spec_one_provider_role<'a>() -> EntrySpec<'a> {
    EntrySpec {
        proof_entry_id: proof::ENTRY_ID_SINGLE_PROVIDER_ROLE,
        requirement: LocalAiRuntimeProviderProofRequirement::OneAiProviderRolePerPhysicalDevice,
        proof_status: LocalAiRuntimeProviderProofStatus::Proved,
        participating_roles: vec![DeviceRuntimeRole::AiProvider],
        accepted_job_classes: all_job_classes(),
        child_safety_priority_proved: false,
        parent_assistant_submission_allowed: false,
        evidence_label: proof::PROOF_ONE_PROVIDER_ROLE,
        capability_requirement: proof::CAPABILITY_ONE_PROVIDER_ROLE,
        proof_requirement: proof::PROOF_ONE_PROVIDER_ROLE,
        claim_boundary: proof::CLAIM_LOCAL_ONLY,
        fallback_behavior: proof::FALLBACK_UNAVAILABLE,
    }
}

fn entry_spec_shared_parent_child<'a>() -> EntrySpec<'a> {
    EntrySpec {
        proof_entry_id: proof::ENTRY_ID_SHARED_PARENT_CHILD_PROVIDER,
        requirement: LocalAiRuntimeProviderProofRequirement::SharedParentChildProvider,
        proof_status: LocalAiRuntimeProviderProofStatus::Proved,
        participating_roles: shared_roles(),
        accepted_job_classes: all_job_classes(),
        child_safety_priority_proved: false,
        parent_assistant_submission_allowed: false,
        evidence_label: proof::PROOF_SHARED_PARENT_CHILD,
        capability_requirement: proof::CAPABILITY_SHARED_PARENT_CHILD,
        proof_requirement: proof::PROOF_SHARED_PARENT_CHILD,
        claim_boundary: proof::CLAIM_SHARED_PROVIDER,
        fallback_behavior: proof::FALLBACK_QUEUE_OR_DEGRADE,
    }
}

fn entry_spec_single_runtime_lane<'a>() -> EntrySpec<'a> {
    EntrySpec {
        proof_entry_id: proof::ENTRY_ID_SINGLE_RUNTIME_LANE,
        requirement: LocalAiRuntimeProviderProofRequirement::SingleLocalRuntimeLane,
        proof_status: LocalAiRuntimeProviderProofStatus::Proved,
        participating_roles: shared_roles(),
        accepted_job_classes: all_job_classes(),
        child_safety_priority_proved: false,
        parent_assistant_submission_allowed: false,
        evidence_label: proof::PROOF_SINGLE_RUNTIME_LANE,
        capability_requirement: proof::CAPABILITY_SINGLE_RUNTIME_LANE,
        proof_requirement: proof::PROOF_SINGLE_RUNTIME_LANE,
        claim_boundary: proof::CLAIM_NO_MODEL_QUALITY,
        fallback_behavior: proof::FALLBACK_BUSY,
    }
}

fn entry_spec_child_safety_priority<'a>() -> EntrySpec<'a> {
    EntrySpec {
        proof_entry_id: proof::ENTRY_ID_CHILD_SAFETY_PRIORITY,
        requirement: LocalAiRuntimeProviderProofRequirement::ChildSafetyPriority,
        proof_status: LocalAiRuntimeProviderProofStatus::Proved,
        participating_roles: vec![DeviceRuntimeRole::ChildAgent, DeviceRuntimeRole::AiProvider],
        accepted_job_classes: all_job_classes(),
        child_safety_priority_proved: true,
        parent_assistant_submission_allowed: false,
        evidence_label: proof::PROOF_CHILD_PRIORITY,
        capability_requirement: proof::CAPABILITY_CHILD_PRIORITY,
        proof_requirement: proof::PROOF_CHILD_PRIORITY,
        claim_boundary: proof::CLAIM_PRIORITY_ONLY,
        fallback_behavior: proof::FALLBACK_PARENT_ASSISTANT_PRIORITY,
    }
}

fn entry_spec_degraded_lifecycle<'a>() -> EntrySpec<'a> {
    EntrySpec {
        proof_entry_id: proof::ENTRY_ID_QUEUED_DEGRADED_LIFECYCLE,
        requirement: LocalAiRuntimeProviderProofRequirement::QueuedDegradedUnavailableLifecycle,
        proof_status: LocalAiRuntimeProviderProofStatus::Degraded,
        participating_roles: vec![
            DeviceRuntimeRole::ParentController,
            DeviceRuntimeRole::AiProvider,
        ],
        accepted_job_classes: vec![LocalAiProviderSchedulerJobClass::ParentAssistant],
        child_safety_priority_proved: false,
        parent_assistant_submission_allowed: true,
        evidence_label: proof::PROOF_LIFECYCLE,
        capability_requirement: proof::CAPABILITY_LIFECYCLE,
        proof_requirement: proof::PROOF_LIFECYCLE,
        claim_boundary: proof::CLAIM_DEGRADED,
        fallback_behavior: proof::FALLBACK_DEGRADED_ANSWER,
    }
}

fn entry_spec_parent_assistant_submit<'a>() -> EntrySpec<'a> {
    EntrySpec {
        proof_entry_id: proof::ENTRY_ID_PARENT_ASSISTANT_SUBMIT,
        requirement: LocalAiRuntimeProviderProofRequirement::ParentAssistantSubmitsWhenAllowed,
        proof_status: LocalAiRuntimeProviderProofStatus::Proved,
        participating_roles: vec![
            DeviceRuntimeRole::ParentController,
            DeviceRuntimeRole::AiProvider,
        ],
        accepted_job_classes: vec![LocalAiProviderSchedulerJobClass::ParentAssistant],
        child_safety_priority_proved: false,
        parent_assistant_submission_allowed: true,
        evidence_label: proof::PROOF_PARENT_ASSISTANT,
        capability_requirement: proof::CAPABILITY_PARENT_ASSISTANT,
        proof_requirement: proof::PROOF_PARENT_ASSISTANT,
        claim_boundary: proof::CLAIM_NO_API_PROVIDER,
        fallback_behavior: proof::FALLBACK_LOCAL_RUNTIME_MISSING,
    }
}

fn entry_spec_no_duplicate_load<'a>() -> EntrySpec<'a> {
    EntrySpec {
        proof_entry_id: proof::ENTRY_ID_NO_DUPLICATE_MODEL_LOAD,
        requirement: LocalAiRuntimeProviderProofRequirement::NoDuplicateLocalModelLoad,
        proof_status: LocalAiRuntimeProviderProofStatus::Proved,
        participating_roles: shared_roles(),
        accepted_job_classes: all_job_classes(),
        child_safety_priority_proved: true,
        parent_assistant_submission_allowed: true,
        evidence_label: proof::PROOF_NO_DUPLICATE_LOAD,
        capability_requirement: proof::CAPABILITY_NO_DUPLICATE_LOAD,
        proof_requirement: proof::PROOF_NO_DUPLICATE_LOAD,
        claim_boundary: proof::CLAIM_NO_CROSS_DEVICE_SHARING,
        fallback_behavior: proof::FALLBACK_BLOCK_DUPLICATE,
    }
}

fn entry_spec_status_hardening<'a>() -> EntrySpec<'a> {
    EntrySpec {
        proof_entry_id: proof::ENTRY_ID_STATUS_CONTRACT_HARDENING,
        requirement: LocalAiRuntimeProviderProofRequirement::ProviderStatusContractHardening,
        proof_status: LocalAiRuntimeProviderProofStatus::Unavailable,
        participating_roles: vec![DeviceRuntimeRole::AiProvider],
        accepted_job_classes: Vec::new(),
        child_safety_priority_proved: false,
        parent_assistant_submission_allowed: false,
        evidence_label: proof::PROOF_STATUS_HARDENING,
        capability_requirement: proof::CAPABILITY_STATUS_HARDENING,
        proof_requirement: proof::PROOF_STATUS_HARDENING,
        claim_boundary: proof::CLAIM_UNAVAILABLE_HONEST,
        fallback_behavior: proof::FALLBACK_UNCONFIGURED,
    }
}

fn shared_roles() -> Vec<DeviceRuntimeRole> {
    vec![
        DeviceRuntimeRole::ParentController,
        DeviceRuntimeRole::ChildAgent,
        DeviceRuntimeRole::AiProvider,
    ]
}

fn all_job_classes() -> Vec<LocalAiProviderSchedulerJobClass> {
    vec![
        LocalAiProviderSchedulerJobClass::ChildSafety,
        LocalAiProviderSchedulerJobClass::ParentAssistant,
        LocalAiProviderSchedulerJobClass::ParentReport,
    ]
}

fn queued_priority_status(
    scheduler_status: &LocalAiProviderSchedulerStatus,
) -> LocalAiProviderSchedulerStatus {
    let mut status = scheduler_status.clone();
    status.lifecycle_state = LocalAiProviderSchedulerLifecycle::Queued;
    status.current_job_class = Some(LocalAiProviderSchedulerJobClass::ParentReport);
    status.queue = LocalAiProviderSchedulerQueue {
        child_safety_queued: 1,
        parent_assistant_queued: 1,
        parent_report_queued: 0,
    };
    status.duplicate_runtime_blocked = true;
    status.degraded_state = LocalAiDegradedState::Overloaded;
    status.unavailable_reason = None;
    status
}

fn degraded_provider_status(
    scheduler_status: &LocalAiProviderSchedulerStatus,
) -> LocalAiProviderSchedulerStatus {
    let mut status = scheduler_status.clone();
    status.lifecycle_state = LocalAiProviderSchedulerLifecycle::Degraded;
    status.current_job_class = None;
    status.queue = LocalAiProviderSchedulerQueue {
        child_safety_queued: 0,
        parent_assistant_queued: 1,
        parent_report_queued: 0,
    };
    status.duplicate_runtime_blocked = true;
    status.degraded_state = LocalAiDegradedState::Overloaded;
    status.unavailable_reason = None;
    status
}

fn unavailable_provider_status(
    generated_at: &LocalAiRuntimeText,
) -> LocalAiProviderSchedulerStatus {
    LocalAiProviderSchedulerStatus {
        physical_device_id: constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string(),
        singleton_scope:
            ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSingletonScope::PhysicalDevice,
        provider_id: constants::local_ai_runtime::PROVIDER_ID_UNCONFIGURED.to_string(),
        runtime_reference_id: constants::local_ai_runtime::RUNTIME_REFERENCE_DEV_UNCONFIGURED
            .to_string(),
        model_id: constants::local_ai_runtime::MODEL_ID_UNCONFIGURED.to_string(),
        model_reference: constants::local_ai_runtime::MODEL_REFERENCE_UNCONFIGURED.to_string(),
        resource_class: LocalAiResourceClass::RemoteUnavailable,
        lifecycle_state: LocalAiProviderSchedulerLifecycle::Unavailable,
        current_job_class: None,
        queue: LocalAiProviderSchedulerQueue::default(),
        duplicate_runtime_blocked: false,
        degraded_state: LocalAiDegradedState::ProviderUnavailable,
        unavailable_reason: Some(
            constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED.to_string(),
        ),
        last_checked_at: generated_at.0.clone(),
    }
}
