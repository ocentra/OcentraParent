use serde::{Deserialize, Serialize};

pub const SCREEN_FAMILY_AI_HUB_ROUTE_SCHEMA_VERSION: u16 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenFamilyAiHubRequestedTask {
    GuidedVisionClassification,
    GuidedMultimodalClassification,
    OcrTextFallback,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenFamilyAiHubCapabilityState {
    Available,
    DisabledByParent,
    HubUnavailable,
    LanProofMissing,
    ResourceExhausted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenFamilyAiHubDegradedState {
    ChildLocalAlreadySelected,
    ParentDisabled,
    HubUnavailable,
    LanProofMissing,
    ResourceExhausted,
    UnsupportedTask,
    CustodyUnsafe,
    ManualRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenFamilyAiHubExecutionState {
    Selected,
    ManualRequired,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenFamilyAiHubTransferMode {
    SummaryOnly,
    RedactedCrop,
    NoTransfer,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenChildLocalAnalysisAttemptState {
    Selected,
    Degraded,
    ManualRequired,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenEvidenceCustodyState {
    LiveLocalChildAgent,
    LiveLanChildAgent,
    ChildDeviceTempQueue,
    ChildDeviceJournal,
    ChildDeviceQueryStore,
    ParentDeviceCache,
    ParentOwnedExport,
    OcentraHostedNonActivity,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenLocalModelProviderKind {
    DeterministicRules,
    LocalOcr,
    LocalVision,
    LocalMultimodal,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenFamilyAiHubCapability {
    pub schema_version: u16,
    pub hub_id: String,
    pub checked_at: String,
    pub capability_state: ScreenFamilyAiHubCapabilityState,
    pub supported_tasks: Vec<ScreenFamilyAiHubRequestedTask>,
    pub model_runtime_ref: Option<String>,
    pub household_route_ref: Option<String>,
    pub custody_state: ScreenEvidenceCustodyState,
    pub no_retention: bool,
    pub local_household_only: bool,
    pub parent_approval_required: bool,
    pub ocentra_hosted_processing_allowed: bool,
    pub raw_image_retention_allowed: bool,
    pub degraded_states: Vec<ScreenFamilyAiHubDegradedState>,
    pub unavailable_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenChildLocalAnalysisAttempt {
    pub attempted: bool,
    pub provider_kind: ScreenLocalModelProviderKind,
    pub execution_state: ScreenChildLocalAnalysisAttemptState,
    pub model_runtime_ref: Option<String>,
    pub degraded_states: Vec<ScreenFamilyAiHubDegradedState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenFamilyAiHubRouteRequest {
    pub route_id: String,
    pub queue_job_id: String,
    pub routed_at: String,
    pub requested_task: ScreenFamilyAiHubRequestedTask,
    pub source_child_local_attempt: ScreenChildLocalAnalysisAttempt,
    pub capability: ScreenFamilyAiHubCapability,
    pub parent_approved_family_hub: bool,
    pub transfer_mode: ScreenFamilyAiHubTransferMode,
    pub source_custody_state: ScreenEvidenceCustodyState,
    pub audit_evidence_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenFamilyAiHubRoute {
    pub schema_version: u16,
    pub route_id: String,
    pub queue_job_id: String,
    pub routed_at: String,
    pub requested_task: ScreenFamilyAiHubRequestedTask,
    pub source_child_local_attempt: ScreenChildLocalAnalysisAttempt,
    pub capability: ScreenFamilyAiHubCapability,
    pub execution_state: ScreenFamilyAiHubExecutionState,
    pub selected_runtime_ref: Option<String>,
    pub transfer_mode: ScreenFamilyAiHubTransferMode,
    pub source_custody_state: ScreenEvidenceCustodyState,
    pub destination_custody_state: ScreenEvidenceCustodyState,
    pub degraded_states: Vec<ScreenFamilyAiHubDegradedState>,
    pub audit_evidence_ids: Vec<String>,
    pub parent_approved_family_hub: bool,
    pub local_provider_attempted: bool,
    pub child_safety_local_fallback_preserved: bool,
    pub summary_first: bool,
    pub redacted_or_cropped_input_required: bool,
    pub raw_full_screenshot_transfer_allowed: bool,
    pub raw_image_retention_allowed: bool,
    pub remote_provider_selected: bool,
    pub remote_api_fallback_allowed: bool,
    pub ocentra_hosted_processing_allowed: bool,
    pub remote_default_for_blocking: bool,
}

pub fn screen_family_ai_hub_capability_is_consistent(value: &ScreenFamilyAiHubCapability) -> bool {
    if value.custody_state != ScreenEvidenceCustodyState::LiveLanChildAgent {
        return false;
    }
    if value.capability_state == ScreenFamilyAiHubCapabilityState::Available {
        return value.model_runtime_ref.is_some()
            && value.household_route_ref.is_some()
            && value.degraded_states.is_empty()
            && value.unavailable_reason.is_none();
    }
    value.model_runtime_ref.is_none()
        && !value.degraded_states.is_empty()
        && value.unavailable_reason.is_some()
}

pub fn screen_child_local_attempt_is_consistent(value: &ScreenChildLocalAnalysisAttempt) -> bool {
    if value.execution_state == ScreenChildLocalAnalysisAttemptState::Selected {
        return value.model_runtime_ref.is_some() && value.degraded_states.is_empty();
    }
    value.model_runtime_ref.is_none() && !value.degraded_states.is_empty()
}

pub fn screen_family_ai_hub_route_is_consistent(value: &ScreenFamilyAiHubRoute) -> bool {
    if !value.local_provider_attempted
        || !value.child_safety_local_fallback_preserved
        || !value.summary_first
        || !value.redacted_or_cropped_input_required
        || value.raw_full_screenshot_transfer_allowed
        || value.raw_image_retention_allowed
        || value.remote_provider_selected
        || value.remote_api_fallback_allowed
        || value.ocentra_hosted_processing_allowed
        || value.remote_default_for_blocking
    {
        return false;
    }

    if value.execution_state == ScreenFamilyAiHubExecutionState::Selected {
        return value.parent_approved_family_hub
            && value.source_child_local_attempt.execution_state
                != ScreenChildLocalAnalysisAttemptState::Selected
            && value.capability.capability_state == ScreenFamilyAiHubCapabilityState::Available
            && value
                .capability
                .supported_tasks
                .contains(&value.requested_task)
            && value.selected_runtime_ref.is_some()
            && value.transfer_mode != ScreenFamilyAiHubTransferMode::NoTransfer
            && value.destination_custody_state == ScreenEvidenceCustodyState::LiveLanChildAgent
            && value.degraded_states.is_empty();
    }
    value.selected_runtime_ref.is_none()
        && value.transfer_mode == ScreenFamilyAiHubTransferMode::NoTransfer
        && !value.degraded_states.is_empty()
}

pub fn plan_screen_family_ai_hub_route(
    request: &ScreenFamilyAiHubRouteRequest,
) -> ScreenFamilyAiHubRoute {
    let selected = screen_family_ai_hub_can_serve(request);
    let degraded_states = if selected {
        Vec::new()
    } else {
        screen_family_ai_hub_degraded_states_for(request)
    };

    ScreenFamilyAiHubRoute {
        schema_version: SCREEN_FAMILY_AI_HUB_ROUTE_SCHEMA_VERSION,
        route_id: request.route_id.clone(),
        queue_job_id: request.queue_job_id.clone(),
        routed_at: request.routed_at.clone(),
        requested_task: request.requested_task.clone(),
        source_child_local_attempt: request.source_child_local_attempt.clone(),
        capability: request.capability.clone(),
        execution_state: if selected {
            ScreenFamilyAiHubExecutionState::Selected
        } else {
            screen_family_ai_hub_execution_state_for(&request.capability)
        },
        selected_runtime_ref: if selected {
            request.capability.model_runtime_ref.clone()
        } else {
            None
        },
        transfer_mode: if selected {
            request.transfer_mode.clone()
        } else {
            ScreenFamilyAiHubTransferMode::NoTransfer
        },
        source_custody_state: request.source_custody_state.clone(),
        destination_custody_state: if selected {
            ScreenEvidenceCustodyState::LiveLanChildAgent
        } else {
            ScreenEvidenceCustodyState::Unavailable
        },
        degraded_states,
        audit_evidence_ids: request.audit_evidence_ids.clone(),
        parent_approved_family_hub: request.parent_approved_family_hub,
        local_provider_attempted: true,
        child_safety_local_fallback_preserved: true,
        summary_first: true,
        redacted_or_cropped_input_required: true,
        raw_full_screenshot_transfer_allowed: false,
        raw_image_retention_allowed: false,
        remote_provider_selected: false,
        remote_api_fallback_allowed: false,
        ocentra_hosted_processing_allowed: false,
        remote_default_for_blocking: false,
    }
}

fn screen_family_ai_hub_can_serve(request: &ScreenFamilyAiHubRouteRequest) -> bool {
    request.parent_approved_family_hub
        && request.source_child_local_attempt.execution_state
            != ScreenChildLocalAnalysisAttemptState::Selected
        && request.capability.capability_state == ScreenFamilyAiHubCapabilityState::Available
        && request
            .capability
            .supported_tasks
            .contains(&request.requested_task)
        && request.transfer_mode != ScreenFamilyAiHubTransferMode::NoTransfer
        && (request.source_custody_state == ScreenEvidenceCustodyState::ChildDeviceTempQueue
            || request.source_custody_state == ScreenEvidenceCustodyState::ChildDeviceJournal)
}

fn screen_family_ai_hub_degraded_states_for(
    request: &ScreenFamilyAiHubRouteRequest,
) -> Vec<ScreenFamilyAiHubDegradedState> {
    if request.source_child_local_attempt.execution_state
        == ScreenChildLocalAnalysisAttemptState::Selected
    {
        return vec![ScreenFamilyAiHubDegradedState::ChildLocalAlreadySelected];
    }
    if !request.parent_approved_family_hub {
        return vec![ScreenFamilyAiHubDegradedState::ParentDisabled];
    }
    if !request
        .capability
        .supported_tasks
        .contains(&request.requested_task)
    {
        return vec![ScreenFamilyAiHubDegradedState::UnsupportedTask];
    }
    if request.source_custody_state != ScreenEvidenceCustodyState::ChildDeviceTempQueue
        && request.source_custody_state != ScreenEvidenceCustodyState::ChildDeviceJournal
    {
        return vec![ScreenFamilyAiHubDegradedState::CustodyUnsafe];
    }
    if request.capability.degraded_states.is_empty() {
        vec![ScreenFamilyAiHubDegradedState::ManualRequired]
    } else {
        request.capability.degraded_states.clone()
    }
}

fn screen_family_ai_hub_execution_state_for(
    capability: &ScreenFamilyAiHubCapability,
) -> ScreenFamilyAiHubExecutionState {
    if capability.capability_state == ScreenFamilyAiHubCapabilityState::HubUnavailable {
        ScreenFamilyAiHubExecutionState::Unavailable
    } else {
        ScreenFamilyAiHubExecutionState::ManualRequired
    }
}

const SCREEN_FAMILY_AI_HUB_ROUTING_GENERATED_TYPESCRIPT: &str = r#"/* generated from crates/screen-ai-core/src/screen_family_ai_hub_routing.rs */

import { type Infer } from '@ocentra-parent/schema-domain/effect';
import {
  ScreenFamilyAiHubRouteRequestSchema,
  type ScreenFamilyAiHubCapability,
  type ScreenFamilyAiHubRoute,
} from '@ocentra-parent/schema-domain/screen-evidence-family-hub-routing';
import { ScreenFamilyAiHubRouteSchemaVersion } from '@ocentra-parent/schema-domain/screen-evidence-family-hub-routing-values';

type ScreenFamilyAiHubRouteRequest = Infer<typeof ScreenFamilyAiHubRouteRequestSchema>;

export function planScreenFamilyAiHubRouteGenerated(
  request: ScreenFamilyAiHubRouteRequest
): ScreenFamilyAiHubRoute {
  const selected = screenFamilyAiHubCanServe(request);
  const degradedStates = selected ? [] : screenFamilyAiHubDegradedStatesFor(request);

  return {
    schemaVersion: ScreenFamilyAiHubRouteSchemaVersion,
    routeId: request.routeId,
    queueJobId: request.queueJobId,
    routedAt: request.routedAt,
    requestedTask: request.requestedTask,
    sourceChildLocalAttempt: request.sourceChildLocalAttempt,
    capability: request.capability,
    executionState: selected ? 'selected' : screenFamilyAiHubExecutionStateFor(request.capability),
    selectedRuntimeRef: selected ? request.capability.modelRuntimeRef : null,
    transferMode: selected ? request.transferMode : 'noTransfer',
    sourceCustodyState: request.sourceCustodyState,
    destinationCustodyState: selected ? 'live-lan-child-agent' : 'unavailable',
    degradedStates,
    auditEvidenceIds: request.auditEvidenceIds,
    parentApprovedFamilyHub: request.parentApprovedFamilyHub,
    localProviderAttempted: true,
    childSafetyLocalFallbackPreserved: true,
    summaryFirst: true,
    redactedOrCroppedInputRequired: true,
    rawFullScreenshotTransferAllowed: false,
    rawImageRetentionAllowed: false,
    remoteProviderSelected: false,
    remoteApiFallbackAllowed: false,
    ocentraHostedProcessingAllowed: false,
    remoteDefaultForBlocking: false,
  };
}

function screenFamilyAiHubCanServe(request: ScreenFamilyAiHubRouteRequest) {
  return (
    request.parentApprovedFamilyHub &&
    request.sourceChildLocalAttempt.executionState !== 'selected' &&
    request.capability.capabilityState === 'available' &&
    request.capability.supportedTasks.includes(request.requestedTask) &&
    request.transferMode !== 'noTransfer' &&
    (request.sourceCustodyState === 'child-device-temp-queue' || request.sourceCustodyState === 'child-device-journal')
  );
}

function screenFamilyAiHubDegradedStatesFor(request: ScreenFamilyAiHubRouteRequest) {
  if (request.sourceChildLocalAttempt.executionState === 'selected') {
    return ['childLocalAlreadySelected'] as const;
  }
  if (!request.parentApprovedFamilyHub) {
    return ['parentDisabled'] as const;
  }
  if (!request.capability.supportedTasks.includes(request.requestedTask)) {
    return ['unsupportedTask'] as const;
  }
  if (
    request.sourceCustodyState !== 'child-device-temp-queue' &&
    request.sourceCustodyState !== 'child-device-journal'
  ) {
    return ['custodyUnsafe'] as const;
  }
  return request.capability.degradedStates.length > 0
    ? request.capability.degradedStates
    : (['manualRequired'] as const);
}

function screenFamilyAiHubExecutionStateFor(capability: ScreenFamilyAiHubCapability) {
  return capability.capabilityState === 'hubUnavailable' ? 'unavailable' : 'manualRequired';
}
"#;

pub fn screen_family_ai_hub_routing_generated_typescript() -> String {
    SCREEN_FAMILY_AI_HUB_ROUTING_GENERATED_TYPESCRIPT.to_string()
}
