import { AgentProtocolDefaults, type AgentEventEnvelope } from '@ocentra-parent/agent-protocol-domain/contracts';
import type { PortalPolicyPreviewReadModel } from '@ocentra-parent/agent-protocol-domain/policy-preview-read-model';
import type { LogFieldValue } from '@ocentra-parent/logging-domain/contracts';
import {
  PortalDetails,
  PortalText,
  PortalTextToken,
  type PortalDetailValue,
} from '@ocentra-parent/portal-domain/contracts';
import { appendDetail } from './detail-list';
import {
  detailFromValue,
  eventStatus,
  notReported,
  payloadDetail,
  type AgentPayloadField,
} from './event-detail-values';

type PolicyPreviewValueResolver = (readModel: PortalPolicyPreviewReadModel) => LogFieldValue | undefined;

const PolicyPreviewValueResolvers = new Map<AgentPayloadField, PolicyPreviewValueResolver>([
  [AgentProtocolDefaults.Field.GeneratedAt, (readModel) => readModel.generatedAt],
  [AgentProtocolDefaults.Field.Custody, (readModel) => readModel.custody],
  [AgentProtocolDefaults.Field.Returned, (readModel) => readModel.returned],
  [AgentProtocolDefaults.Field.CapabilityStatus, (readModel) => readModel.capabilityStatus],
  [AgentProtocolDefaults.Field.PolicyPreviewId, (readModel) => linkedPolicyDecision(readModel.previewId)],
  [AgentProtocolDefaults.Field.LatestEventId, (readModel) => linkedActivityRecord(readModel.latestEventId)],
  [AgentProtocolDefaults.Field.LatestObservedAt, (readModel) => readModel.latestObservedAt],
  [AgentProtocolDefaults.Field.TargetId, (readModel) => linkedActivityRecord(readModel.targetId)],
  [AgentProtocolDefaults.Field.PolicyTargetType, (readModel) => readModel.targetType],
  [AgentProtocolDefaults.Field.PolicyTargetValue, (readModel) => readModel.targetValue],
  [AgentProtocolDefaults.Field.PolicyEvidenceReferenceCount, (readModel) => readModel.evidenceReferenceCount],
  [
    AgentProtocolDefaults.Field.PolicyParentRuleContextReferenceCount,
    (readModel) => readModel.parentRuleContextReferenceCount,
  ],
  [AgentProtocolDefaults.Field.PolicyParentRuleContextRefIds, (readModel) => readModel.parentRuleContextRefIds],
  [AgentProtocolDefaults.Field.PolicyDecisionId, (readModel) => linkedPolicyDecision(readModel.decisionId)],
  [AgentProtocolDefaults.Field.PolicyAction, (readModel) => readModel.decisionAction],
  [AgentProtocolDefaults.Field.PolicyReasonCodes, (readModel) => readModel.reasonCodes],
  [AgentProtocolDefaults.Field.PolicyRuleIds, (readModel) => readModel.ruleIds],
  [AgentProtocolDefaults.Field.LocalAiResultId, (readModel) => readModel.localAiResultId],
  [AgentProtocolDefaults.Field.PolicyDryRun, (readModel) => policyMode(readModel.dryRun)],
  [AgentProtocolDefaults.Field.PolicyHandoffState, (readModel) => policyMode(readModel.dryRun)],
  [AgentProtocolDefaults.Field.NetworkEvidenceGrade, (readModel) => readModel.networkEvidenceGrade],
  [AgentProtocolDefaults.Field.NetworkRequestedPolicyAction, (readModel) => readModel.networkRequestedPolicyAction],
  [AgentProtocolDefaults.Field.NetworkMappedPolicyAction, (readModel) => readModel.networkMappedPolicyAction],
  [AgentProtocolDefaults.Field.NetworkPolicyMappingMode, (readModel) => readModel.networkPolicyMappingMode],
  [AgentProtocolDefaults.Field.NetworkAdapterActionAuthorized, (readModel) => readModel.networkAdapterActionAuthorized],
  [
    AgentProtocolDefaults.Field.NetworkEnforcementCommandAuthorized,
    (readModel) => readModel.networkEnforcementCommandAuthorized,
  ],
]);

export function appendReadModelDetails(
  metadata: HTMLDListElement,
  event: AgentEventEnvelope | null,
  readModel: PortalPolicyPreviewReadModel | null
): void {
  appendDetail(metadata, PortalDetails.PreviewStatus, eventStatus(event));
  appendDetail(
    metadata,
    PortalDetails.GeneratedAt,
    readModelDetail(readModel, AgentProtocolDefaults.Field.GeneratedAt)
  );
  appendDetail(metadata, PortalDetails.Custody, readModelDetail(readModel, AgentProtocolDefaults.Field.Custody));
  appendDetail(metadata, PortalDetails.RowsReturned, readModelDetail(readModel, AgentProtocolDefaults.Field.Returned));
  appendDetail(
    metadata,
    PortalDetails.Capability,
    readModelDetail(readModel, AgentProtocolDefaults.Field.CapabilityStatus)
  );
  appendDetail(
    metadata,
    PortalDetails.PolicyPreview,
    readModelDetail(readModel, AgentProtocolDefaults.Field.PolicyPreviewId)
  );
  appendDetail(metadata, PortalDetails.EventId, readModelDetail(readModel, AgentProtocolDefaults.Field.LatestEventId));
  appendDetail(
    metadata,
    PortalDetails.LastObserved,
    readModelDetail(readModel, AgentProtocolDefaults.Field.LatestObservedAt)
  );
  appendDetail(metadata, PortalDetails.TargetId, readModelDetail(readModel, AgentProtocolDefaults.Field.TargetId));
  appendDetail(
    metadata,
    PortalDetails.TargetType,
    readModelDetail(readModel, AgentProtocolDefaults.Field.PolicyTargetType)
  );
  appendDetail(
    metadata,
    PortalDetails.TargetValue,
    readModelDetail(readModel, AgentProtocolDefaults.Field.PolicyTargetValue)
  );
  if (readModel === null && event !== null) {
    appendDetail(metadata, PortalDetails.Reason, payloadDetail(event, AgentProtocolDefaults.Field.Reason));
  }
}

export function appendDecisionPreviewDetails(
  metadata: HTMLDListElement,
  readModel: PortalPolicyPreviewReadModel | null
): void {
  appendDetail(
    metadata,
    PortalDetails.DecisionId,
    readModelDetail(readModel, AgentProtocolDefaults.Field.PolicyDecisionId)
  );
  appendDetail(
    metadata,
    PortalDetails.DecisionAction,
    readModelDetail(readModel, AgentProtocolDefaults.Field.PolicyAction)
  );
  appendDetail(
    metadata,
    PortalDetails.ReasonCodes,
    readModelDetail(readModel, AgentProtocolDefaults.Field.PolicyReasonCodes)
  );
  appendDetail(metadata, PortalDetails.RuleIds, readModelDetail(readModel, AgentProtocolDefaults.Field.PolicyRuleIds));
  appendDetail(
    metadata,
    PortalDetails.ParentRuleContextReferences,
    readModelDetail(readModel, AgentProtocolDefaults.Field.PolicyParentRuleContextReferenceCount)
  );
  appendDetail(
    metadata,
    PortalDetails.ParentRuleContextRefIds,
    readModelDetail(readModel, AgentProtocolDefaults.Field.PolicyParentRuleContextRefIds)
  );
  appendDetail(
    metadata,
    PortalDetails.EvidenceReferences,
    readModelDetail(readModel, AgentProtocolDefaults.Field.PolicyEvidenceReferenceCount)
  );
  appendDetail(
    metadata,
    PortalDetails.LocalAiResult,
    readModelDetail(readModel, AgentProtocolDefaults.Field.LocalAiResultId)
  );
  appendDetail(metadata, PortalDetails.DryRun, readModelDetail(readModel, AgentProtocolDefaults.Field.PolicyDryRun));
  appendDetail(
    metadata,
    PortalDetails.EnforcementHandoff,
    readModelDetail(readModel, AgentProtocolDefaults.Field.PolicyHandoffState)
  );
  appendDetail(
    metadata,
    PortalDetails.NetworkEvidenceGrade,
    readModelDetail(readModel, AgentProtocolDefaults.Field.NetworkEvidenceGrade)
  );
  appendDetail(
    metadata,
    PortalDetails.NetworkRequestedPolicyAction,
    readModelDetail(readModel, AgentProtocolDefaults.Field.NetworkRequestedPolicyAction)
  );
  appendDetail(
    metadata,
    PortalDetails.NetworkMappedPolicyAction,
    readModelDetail(readModel, AgentProtocolDefaults.Field.NetworkMappedPolicyAction)
  );
  appendDetail(
    metadata,
    PortalDetails.NetworkPolicyMappingMode,
    readModelDetail(readModel, AgentProtocolDefaults.Field.NetworkPolicyMappingMode)
  );
  appendDetail(
    metadata,
    PortalDetails.NetworkAdapterAuthorization,
    readModelDetail(readModel, AgentProtocolDefaults.Field.NetworkAdapterActionAuthorized)
  );
  appendDetail(
    metadata,
    PortalDetails.NetworkEnforcementAuthorization,
    readModelDetail(readModel, AgentProtocolDefaults.Field.NetworkEnforcementCommandAuthorized)
  );
  appendDetail(metadata, PortalDetails.UnknownState, notReported());
}

function readModelDetail(readModel: PortalPolicyPreviewReadModel | null, field: AgentPayloadField): PortalDetailValue {
  if (readModel === null) {
    return notReported();
  }
  return detailFromValue(PolicyPreviewValueResolvers.get(field)?.(readModel));
}

function policyMode(dryRun: boolean | null): LogFieldValue {
  if (dryRun === false) {
    return PortalText.Resolve(PortalTextToken.PolicyModeActive);
  }
  return PortalText.Resolve(PortalTextToken.PolicyModeAdvisory);
}

function linkedPolicyDecision(value: LogFieldValue | undefined): LogFieldValue | undefined {
  if (value === undefined || value === null) {
    return value;
  }
  return PortalText.Resolve(PortalTextToken.ProductSurfaceWired);
}

function linkedActivityRecord(value: LogFieldValue | undefined): LogFieldValue | undefined {
  if (value === undefined || value === null) {
    return value;
  }
  return PortalText.Resolve(PortalTextToken.ProductStatusLocalOnly);
}
