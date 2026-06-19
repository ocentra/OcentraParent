import { type LogFields } from '@ocentra-parent/logging-domain/contracts';
import { type Infer, NonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentProtocolDefaults } from './defaults';

type AgentProtocolLogFields = LogFields;

const PolicyPreviewField = {
  SchemaVersion: AgentProtocolDefaults.Field.SchemaVersion,
  GeneratedAt: AgentProtocolDefaults.Field.GeneratedAt,
  Custody: AgentProtocolDefaults.Field.Custody,
  Limit: AgentProtocolDefaults.Field.Limit,
  Returned: AgentProtocolDefaults.Field.Returned,
  CapabilityStatus: AgentProtocolDefaults.Field.CapabilityStatus,
  PreviewId: AgentProtocolDefaults.Field.PolicyPreviewId,
  LatestEventId: AgentProtocolDefaults.Field.LatestEventId,
  LatestObservedAt: AgentProtocolDefaults.Field.LatestObservedAt,
  TargetId: AgentProtocolDefaults.Field.TargetId,
  TargetType: AgentProtocolDefaults.Field.PolicyTargetType,
  TargetValue: AgentProtocolDefaults.Field.PolicyTargetValue,
  EvidenceReferenceCount: AgentProtocolDefaults.Field.PolicyEvidenceReferenceCount,
  ParentRuleContextReferenceCount: AgentProtocolDefaults.Field.PolicyParentRuleContextReferenceCount,
  ParentRuleContextRefIds: AgentProtocolDefaults.Field.PolicyParentRuleContextRefIds,
  DecisionId: AgentProtocolDefaults.Field.PolicyDecisionId,
  DecisionAction: AgentProtocolDefaults.Field.PolicyAction,
  ReasonCodes: AgentProtocolDefaults.Field.PolicyReasonCodes,
  RuleIds: AgentProtocolDefaults.Field.PolicyRuleIds,
  LocalAiResultId: AgentProtocolDefaults.Field.LocalAiResultId,
  DryRun: AgentProtocolDefaults.Field.PolicyDryRun,
  HandoffState: AgentProtocolDefaults.Field.PolicyHandoffState,
  PreviewSaveState: AgentProtocolDefaults.Field.PolicyPreviewSaveState,
  PreviewManualReviewState: AgentProtocolDefaults.Field.PolicyPreviewManualReviewState,
  PreviewTargetState: AgentProtocolDefaults.Field.PolicyPreviewTargetState,
  PreviewTargetExplanationCode: AgentProtocolDefaults.Field.PolicyPreviewTargetExplanationCode,
  PreviewFindingKinds: AgentProtocolDefaults.Field.PolicyPreviewFindingKinds,
  SourceStatus: AgentProtocolDefaults.Field.PolicySourceStatus,
  SourceSurface: AgentProtocolDefaults.Field.PolicySourceSurface,
  RequestOrigin: AgentProtocolDefaults.Field.PolicyRequestOrigin,
  AssistantConfirmationState: AgentProtocolDefaults.Field.PolicyAssistantConfirmationState,
  RequestStatus: AgentProtocolDefaults.Field.PolicyRequestStatus,
  ApprovalId: AgentProtocolDefaults.Field.PolicyApprovalId,
  OverrideId: AgentProtocolDefaults.Field.PolicyOverrideId,
  ReplayOfApprovalId: AgentProtocolDefaults.Field.PolicyReplayOfApprovalId,
  ReviewedByActorId: AgentProtocolDefaults.Field.PolicyReviewedByActorId,
  ReviewedByActorRole: AgentProtocolDefaults.Field.PolicyReviewedByActorRole,
  ReviewedAt: AgentProtocolDefaults.Field.PolicyReviewedAt,
  AuditReferenceId: AgentProtocolDefaults.Field.PolicyAuditReferenceId,
  NetworkEvidenceGrade: AgentProtocolDefaults.Field.NetworkEvidenceGrade,
  NetworkRequestedPolicyAction: AgentProtocolDefaults.Field.NetworkRequestedPolicyAction,
  NetworkMappedPolicyAction: AgentProtocolDefaults.Field.NetworkMappedPolicyAction,
  NetworkPolicyMappingMode: AgentProtocolDefaults.Field.NetworkPolicyMappingMode,
  NetworkAdapterActionAuthorized: AgentProtocolDefaults.Field.NetworkAdapterActionAuthorized,
  NetworkEnforcementCommandAuthorized: AgentProtocolDefaults.Field.NetworkEnforcementCommandAuthorized,
} as const;

const PolicyPreviewDefaults = AgentProtocolDefaults.PolicyPreview;

const NullableTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);
const NullableSchemaVersionSchema = Schema.Union(NonEmptyStringSchema, Schema.Number, Schema.Null);
const NullableNumberSchema = Schema.Union(Schema.Number, Schema.Null);
const NullableBooleanSchema = Schema.Union(Schema.Boolean, Schema.Null);
const FalseOrNullSchema = Schema.Union(Schema.Literal(false), Schema.Null);
const NullablePolicyTargetTypeSchema = Schema.Union(
  Schema.Literal(
    PolicyPreviewDefaults.TargetType.App,
    PolicyPreviewDefaults.TargetType.Device,
    PolicyPreviewDefaults.TargetType.Url,
    PolicyPreviewDefaults.TargetType.Domain,
    PolicyPreviewDefaults.TargetType.Site,
    PolicyPreviewDefaults.TargetType.Category,
    PolicyPreviewDefaults.TargetType.NetworkDomain,
    PolicyPreviewDefaults.TargetType.Unknown
  ),
  Schema.Null
);
const NullablePolicyActionSchema = Schema.Union(
  Schema.Literal(
    PolicyPreviewDefaults.Action.Allow,
    PolicyPreviewDefaults.Action.None,
    PolicyPreviewDefaults.Action.AskParent,
    PolicyPreviewDefaults.Action.WarnChild,
    PolicyPreviewDefaults.Action.Monitor,
    PolicyPreviewDefaults.Action.Limit,
    PolicyPreviewDefaults.Action.Block,
    PolicyPreviewDefaults.Action.ManualReview
  ),
  Schema.Null
);
const NullableNetworkEvidenceGradeSchema = Schema.Union(
  Schema.Literal(
    PolicyPreviewDefaults.EvidenceGrade.A,
    PolicyPreviewDefaults.EvidenceGrade.B,
    PolicyPreviewDefaults.EvidenceGrade.C,
    PolicyPreviewDefaults.EvidenceGrade.D
  ),
  Schema.Null
);
const NullableNetworkPolicyMappingModeSchema = Schema.Union(
  Schema.Literal(
    PolicyPreviewDefaults.MappingMode.ParentReview,
    PolicyPreviewDefaults.MappingMode.ObserveOnly,
    PolicyPreviewDefaults.MappingMode.DryRun,
    PolicyPreviewDefaults.MappingMode.ManualRequired,
    PolicyPreviewDefaults.MappingMode.AdapterUnavailable
  ),
  Schema.Null
);
const NullablePolicyHandoffStateSchema = Schema.Union(
  Schema.Literal(PolicyPreviewDefaults.HandoffState.DisabledPreviewOnly),
  Schema.Null
);
const NullablePolicyPreviewSaveStateSchema = Schema.Union(
  Schema.Literal(
    PolicyPreviewDefaults.SaveState.PreviewRequired,
    PolicyPreviewDefaults.SaveState.ReadyToSave,
    PolicyPreviewDefaults.SaveState.Blocked
  ),
  Schema.Null
);
const NullablePolicyPreviewManualReviewStateSchema = Schema.Union(
  Schema.Literal(
    PolicyPreviewDefaults.ManualReviewState.Required,
    PolicyPreviewDefaults.ManualReviewState.NotRequired
  ),
  Schema.Null
);
const NullablePolicyPreviewTargetStateSchema = Schema.Union(
  Schema.Literal(
    PolicyPreviewDefaults.TargetState.Supported,
    PolicyPreviewDefaults.TargetState.Unsupported,
    PolicyPreviewDefaults.TargetState.ManualRequired,
    PolicyPreviewDefaults.TargetState.Offline,
    PolicyPreviewDefaults.TargetState.Stale
  ),
  Schema.Null
);
const NullablePolicySourceStatusSchema = Schema.Union(
  Schema.Literal(
    PolicyPreviewDefaults.SourceStatus.Draft,
    PolicyPreviewDefaults.SourceStatus.Preview,
    PolicyPreviewDefaults.SourceStatus.Confirmed,
    PolicyPreviewDefaults.SourceStatus.Queued,
    PolicyPreviewDefaults.SourceStatus.Delivered,
    PolicyPreviewDefaults.SourceStatus.Acknowledged,
    PolicyPreviewDefaults.SourceStatus.Active,
    PolicyPreviewDefaults.SourceStatus.PartiallyActive,
    PolicyPreviewDefaults.SourceStatus.Rejected,
    PolicyPreviewDefaults.SourceStatus.Superseded,
    PolicyPreviewDefaults.SourceStatus.RolledBack,
    PolicyPreviewDefaults.SourceStatus.Stale,
    PolicyPreviewDefaults.SourceStatus.Expired,
    PolicyPreviewDefaults.SourceStatus.ManualRequired
  ),
  Schema.Null
);
const NullablePolicySourceSurfaceSchema = Schema.Union(
  Schema.Literal(
    PolicyPreviewDefaults.SourceSurface.ParentPortal,
    PolicyPreviewDefaults.SourceSurface.ParentCompanion,
    PolicyPreviewDefaults.SourceSurface.AiPreview,
    PolicyPreviewDefaults.SourceSurface.DomainCache
  ),
  Schema.Null
);
const NullablePolicyRequestOriginSchema = Schema.Union(
  Schema.Literal(PolicyPreviewDefaults.RequestOrigin.Child, PolicyPreviewDefaults.RequestOrigin.AssistantDraft),
  Schema.Null
);
const NullablePolicyAssistantConfirmationStateSchema = Schema.Union(
  Schema.Literal(
    PolicyPreviewDefaults.AssistantConfirmationState.NotRequired,
    PolicyPreviewDefaults.AssistantConfirmationState.ParentConfirmationRequired,
    PolicyPreviewDefaults.AssistantConfirmationState.ParentConfirmed
  ),
  Schema.Null
);
const NullablePolicyRequestStatusSchema = Schema.Union(
  Schema.Literal(
    PolicyPreviewDefaults.RequestStatus.PreviewOnly,
    PolicyPreviewDefaults.RequestStatus.PendingParentReview,
    PolicyPreviewDefaults.RequestStatus.Approved,
    PolicyPreviewDefaults.RequestStatus.Denied,
    PolicyPreviewDefaults.RequestStatus.Modified,
    PolicyPreviewDefaults.RequestStatus.Expired,
    PolicyPreviewDefaults.RequestStatus.ReplayRejected
  ),
  Schema.Null
);

const PortalPolicyPreviewReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: NullableSchemaVersionSchema,
    generatedAt: NullableTextSchema,
    custody: NullableTextSchema,
    limit: NullableNumberSchema,
    returned: Schema.Number,
    capabilityStatus: NullableTextSchema,
    previewId: NullableTextSchema,
    latestEventId: NullableTextSchema,
    latestObservedAt: NullableTextSchema,
    targetId: NullableTextSchema,
    targetType: NullablePolicyTargetTypeSchema,
    targetValue: NullableTextSchema,
    evidenceReferenceCount: NullableNumberSchema,
    parentRuleContextReferenceCount: NullableNumberSchema,
    parentRuleContextRefIds: NullableTextSchema,
    decisionId: NullableTextSchema,
    decisionAction: NullablePolicyActionSchema,
    reasonCodes: NullableTextSchema,
    ruleIds: NullableTextSchema,
    localAiResultId: NullableTextSchema,
    dryRun: NullableBooleanSchema,
    enforcementHandoffState: NullablePolicyHandoffStateSchema,
    policyPreviewSaveState: NullablePolicyPreviewSaveStateSchema,
    policyPreviewManualReviewState: NullablePolicyPreviewManualReviewStateSchema,
    policyPreviewTargetState: NullablePolicyPreviewTargetStateSchema,
    policyPreviewTargetExplanationCode: NullableTextSchema,
    policyPreviewFindingKinds: NullableTextSchema,
    policySourceStatus: NullablePolicySourceStatusSchema,
    policySourceSurface: NullablePolicySourceSurfaceSchema,
    policyRequestOrigin: NullablePolicyRequestOriginSchema,
    policyAssistantConfirmationState: NullablePolicyAssistantConfirmationStateSchema,
    policyRequestStatus: NullablePolicyRequestStatusSchema,
    policyApprovalId: NullableTextSchema,
    policyOverrideId: NullableTextSchema,
    policyReplayOfApprovalId: NullableTextSchema,
    policyReviewedByActorId: NullableTextSchema,
    policyReviewedByActorRole: NullableTextSchema,
    policyReviewedAt: NullableTextSchema,
    policyAuditReferenceId: NullableTextSchema,
    networkEvidenceGrade: NullableNetworkEvidenceGradeSchema,
    networkRequestedPolicyAction: NullablePolicyActionSchema,
    networkMappedPolicyAction: NullablePolicyActionSchema,
    networkPolicyMappingMode: NullableNetworkPolicyMappingModeSchema,
    networkAdapterActionAuthorized: FalseOrNullSchema,
    networkEnforcementCommandAuthorized: FalseOrNullSchema,
  }).pipe(
    Schema.filter(
      (readModel) =>
        !hasNetworkPolicyFields(readModel) ||
        (readModel.dryRun === true &&
          readModel.enforcementHandoffState === PolicyPreviewDefaults.HandoffState.DisabledPreviewOnly) ||
        PolicyPreviewDefaults.ValidationMessage.DryRunPreviewOnlyHandoffRequired
    )
  )
);

export type PortalPolicyPreviewReadModel = Infer<typeof PortalPolicyPreviewReadModelSchema>;

export function parsePolicyPreviewReadModel(payload: AgentProtocolLogFields): PortalPolicyPreviewReadModel | null {
  const parsed = PortalPolicyPreviewReadModelSchema.safeParse({
    schemaVersion: valueOrNull(payload[PolicyPreviewField.SchemaVersion]),
    generatedAt: valueOrNull(payload[PolicyPreviewField.GeneratedAt]),
    custody: valueOrNull(payload[PolicyPreviewField.Custody]),
    limit: valueOrNull(payload[PolicyPreviewField.Limit]),
    returned: payload[PolicyPreviewField.Returned],
    capabilityStatus: valueOrNull(payload[PolicyPreviewField.CapabilityStatus]),
    previewId: valueOrNull(payload[PolicyPreviewField.PreviewId]),
    latestEventId: valueOrNull(payload[PolicyPreviewField.LatestEventId]),
    latestObservedAt: valueOrNull(payload[PolicyPreviewField.LatestObservedAt]),
    targetId: valueOrNull(payload[PolicyPreviewField.TargetId]),
    targetType: valueOrNull(payload[PolicyPreviewField.TargetType]),
    targetValue: valueOrNull(payload[PolicyPreviewField.TargetValue]),
    evidenceReferenceCount: valueOrNull(payload[PolicyPreviewField.EvidenceReferenceCount]),
    parentRuleContextReferenceCount: valueOrNull(payload[PolicyPreviewField.ParentRuleContextReferenceCount]),
    parentRuleContextRefIds: valueOrNull(payload[PolicyPreviewField.ParentRuleContextRefIds]),
    decisionId: valueOrNull(payload[PolicyPreviewField.DecisionId]),
    decisionAction: valueOrNull(payload[PolicyPreviewField.DecisionAction]),
    reasonCodes: valueOrNull(payload[PolicyPreviewField.ReasonCodes]),
    ruleIds: valueOrNull(payload[PolicyPreviewField.RuleIds]),
    localAiResultId: valueOrNull(payload[PolicyPreviewField.LocalAiResultId]),
    dryRun: valueOrNull(payload[PolicyPreviewField.DryRun]),
    enforcementHandoffState: valueOrNull(payload[PolicyPreviewField.HandoffState]),
    policyPreviewSaveState: valueOrNull(payload[PolicyPreviewField.PreviewSaveState]),
    policyPreviewManualReviewState: valueOrNull(payload[PolicyPreviewField.PreviewManualReviewState]),
    policyPreviewTargetState: valueOrNull(payload[PolicyPreviewField.PreviewTargetState]),
    policyPreviewTargetExplanationCode: valueOrNull(payload[PolicyPreviewField.PreviewTargetExplanationCode]),
    policyPreviewFindingKinds: valueOrNull(payload[PolicyPreviewField.PreviewFindingKinds]),
    policySourceStatus: valueOrNull(payload[PolicyPreviewField.SourceStatus]),
    policySourceSurface: valueOrNull(payload[PolicyPreviewField.SourceSurface]),
    policyRequestOrigin: valueOrNull(payload[PolicyPreviewField.RequestOrigin]),
    policyAssistantConfirmationState: valueOrNull(payload[PolicyPreviewField.AssistantConfirmationState]),
    policyRequestStatus: valueOrNull(payload[PolicyPreviewField.RequestStatus]),
    policyApprovalId: valueOrNull(payload[PolicyPreviewField.ApprovalId]),
    policyOverrideId: valueOrNull(payload[PolicyPreviewField.OverrideId]),
    policyReplayOfApprovalId: valueOrNull(payload[PolicyPreviewField.ReplayOfApprovalId]),
    policyReviewedByActorId: valueOrNull(payload[PolicyPreviewField.ReviewedByActorId]),
    policyReviewedByActorRole: valueOrNull(payload[PolicyPreviewField.ReviewedByActorRole]),
    policyReviewedAt: valueOrNull(payload[PolicyPreviewField.ReviewedAt]),
    policyAuditReferenceId: valueOrNull(payload[PolicyPreviewField.AuditReferenceId]),
    networkEvidenceGrade: valueOrNull(payload[PolicyPreviewField.NetworkEvidenceGrade]),
    networkRequestedPolicyAction: valueOrNull(payload[PolicyPreviewField.NetworkRequestedPolicyAction]),
    networkMappedPolicyAction: valueOrNull(payload[PolicyPreviewField.NetworkMappedPolicyAction]),
    networkPolicyMappingMode: valueOrNull(payload[PolicyPreviewField.NetworkPolicyMappingMode]),
    networkAdapterActionAuthorized: valueOrNull(payload[PolicyPreviewField.NetworkAdapterActionAuthorized]),
    networkEnforcementCommandAuthorized: valueOrNull(
      payload[PolicyPreviewField.NetworkEnforcementCommandAuthorized]
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

interface PortalPolicyPreviewNetworkFields {
  readonly networkEvidenceGrade: unknown | null;
  readonly networkRequestedPolicyAction: unknown | null;
  readonly networkMappedPolicyAction: unknown | null;
  readonly networkPolicyMappingMode: unknown | null;
}

function hasNetworkPolicyFields(readModel: PortalPolicyPreviewNetworkFields): boolean {
  return (
    readModel.networkEvidenceGrade !== null ||
    readModel.networkRequestedPolicyAction !== null ||
    readModel.networkMappedPolicyAction !== null ||
    readModel.networkPolicyMappingMode !== null
  );
}
