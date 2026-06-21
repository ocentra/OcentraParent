import { type LogFields } from '@ocentra-parent/schema-domain/logging-contracts';
import {
  AgentPolicyPreviewField,
  PortalPolicyPreviewReadModelSchema,
  type PortalPolicyPreviewReadModel,
} from '@ocentra-parent/schema-domain/agent-policy-preview-read-model';

export function parsePolicyPreviewReadModel(payload: LogFields): PortalPolicyPreviewReadModel | null {
  const parsed = PortalPolicyPreviewReadModelSchema.safeParse({
    schemaVersion: valueOrNull(payload[AgentPolicyPreviewField.SchemaVersion]),
    generatedAt: valueOrNull(payload[AgentPolicyPreviewField.GeneratedAt]),
    custody: valueOrNull(payload[AgentPolicyPreviewField.Custody]),
    limit: valueOrNull(payload[AgentPolicyPreviewField.Limit]),
    returned: payload[AgentPolicyPreviewField.Returned],
    capabilityStatus: valueOrNull(payload[AgentPolicyPreviewField.CapabilityStatus]),
    previewId: valueOrNull(payload[AgentPolicyPreviewField.PreviewId]),
    latestEventId: valueOrNull(payload[AgentPolicyPreviewField.LatestEventId]),
    latestObservedAt: valueOrNull(payload[AgentPolicyPreviewField.LatestObservedAt]),
    targetId: valueOrNull(payload[AgentPolicyPreviewField.TargetId]),
    targetType: valueOrNull(payload[AgentPolicyPreviewField.TargetType]),
    targetValue: valueOrNull(payload[AgentPolicyPreviewField.TargetValue]),
    evidenceReferenceCount: valueOrNull(payload[AgentPolicyPreviewField.EvidenceReferenceCount]),
    parentRuleContextReferenceCount: valueOrNull(payload[AgentPolicyPreviewField.ParentRuleContextReferenceCount]),
    parentRuleContextRefIds: valueOrNull(payload[AgentPolicyPreviewField.ParentRuleContextRefIds]),
    decisionId: valueOrNull(payload[AgentPolicyPreviewField.DecisionId]),
    decisionAction: valueOrNull(payload[AgentPolicyPreviewField.DecisionAction]),
    reasonCodes: valueOrNull(payload[AgentPolicyPreviewField.ReasonCodes]),
    ruleIds: valueOrNull(payload[AgentPolicyPreviewField.RuleIds]),
    localAiResultId: valueOrNull(payload[AgentPolicyPreviewField.LocalAiResultId]),
    dryRun: valueOrNull(payload[AgentPolicyPreviewField.DryRun]),
    enforcementHandoffState: valueOrNull(payload[AgentPolicyPreviewField.HandoffState]),
    policyPreviewSaveState: valueOrNull(payload[AgentPolicyPreviewField.PreviewSaveState]),
    policyPreviewManualReviewState: valueOrNull(payload[AgentPolicyPreviewField.PreviewManualReviewState]),
    policyPreviewTargetState: valueOrNull(payload[AgentPolicyPreviewField.PreviewTargetState]),
    policyPreviewTargetExplanationCode: valueOrNull(payload[AgentPolicyPreviewField.PreviewTargetExplanationCode]),
    policyPreviewFindingKinds: valueOrNull(payload[AgentPolicyPreviewField.PreviewFindingKinds]),
    policySourceStatus: valueOrNull(payload[AgentPolicyPreviewField.SourceStatus]),
    policySourceSurface: valueOrNull(payload[AgentPolicyPreviewField.SourceSurface]),
    policyRequestOrigin: valueOrNull(payload[AgentPolicyPreviewField.RequestOrigin]),
    policyAssistantConfirmationState: valueOrNull(payload[AgentPolicyPreviewField.AssistantConfirmationState]),
    policyRequestStatus: valueOrNull(payload[AgentPolicyPreviewField.RequestStatus]),
    policyApprovalId: valueOrNull(payload[AgentPolicyPreviewField.ApprovalId]),
    policyOverrideId: valueOrNull(payload[AgentPolicyPreviewField.OverrideId]),
    policyReplayOfApprovalId: valueOrNull(payload[AgentPolicyPreviewField.ReplayOfApprovalId]),
    policyReviewedByActorId: valueOrNull(payload[AgentPolicyPreviewField.ReviewedByActorId]),
    policyReviewedByActorRole: valueOrNull(payload[AgentPolicyPreviewField.ReviewedByActorRole]),
    policyReviewedAt: valueOrNull(payload[AgentPolicyPreviewField.ReviewedAt]),
    policyAuditReferenceId: valueOrNull(payload[AgentPolicyPreviewField.AuditReferenceId]),
    networkEvidenceGrade: valueOrNull(payload[AgentPolicyPreviewField.NetworkEvidenceGrade]),
    networkRequestedPolicyAction: valueOrNull(payload[AgentPolicyPreviewField.NetworkRequestedPolicyAction]),
    networkMappedPolicyAction: valueOrNull(payload[AgentPolicyPreviewField.NetworkMappedPolicyAction]),
    networkPolicyMappingMode: valueOrNull(payload[AgentPolicyPreviewField.NetworkPolicyMappingMode]),
    networkAdapterActionAuthorized: valueOrNull(payload[AgentPolicyPreviewField.NetworkAdapterActionAuthorized]),
    networkEnforcementCommandAuthorized: valueOrNull(
      payload[AgentPolicyPreviewField.NetworkEnforcementCommandAuthorized]
    ),
  });

  if (!parsed.success) {
    return null;
  }
  return parsed.data;
}

function valueOrNull(value: unknown): unknown {
  return value === undefined ? null : value;
}
