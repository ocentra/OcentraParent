import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';
import { ActivityNetworkEvidenceGradeSchema } from './network-contracts';

export const AgentPolicyPreviewField = {
  SchemaVersion: 'schemaVersion',
  GeneratedAt: 'generatedAt',
  Custody: 'custody',
  Limit: 'limit',
  Returned: 'returned',
  CapabilityStatus: 'capabilityStatus',
  PreviewId: 'policyPreviewId',
  LatestEventId: 'latestEventId',
  LatestObservedAt: 'latestObservedAt',
  TargetId: 'targetId',
  TargetType: 'targetType',
  TargetValue: 'targetValue',
  EvidenceReferenceCount: 'evidenceReferenceCount',
  ParentRuleContextReferenceCount: 'parentRuleContextReferenceCount',
  ParentRuleContextRefIds: 'parentRuleContextRefIds',
  DecisionId: 'policyDecisionId',
  DecisionAction: 'policyAction',
  ReasonCodes: 'reasonCodes',
  RuleIds: 'policyRuleIds',
  LocalAiResultId: 'localAiResultId',
  DryRun: 'dryRun',
  HandoffState: 'enforcementHandoffState',
  PreviewSaveState: 'policyPreviewSaveState',
  PreviewManualReviewState: 'policyPreviewManualReviewState',
  PreviewTargetState: 'policyPreviewTargetState',
  PreviewTargetExplanationCode: 'policyPreviewTargetExplanationCode',
  PreviewFindingKinds: 'policyPreviewFindingKinds',
  SourceStatus: 'policySourceStatus',
  SourceSurface: 'policySourceSurface',
  RequestOrigin: 'policyRequestOrigin',
  AssistantConfirmationState: 'policyAssistantConfirmationState',
  RequestStatus: 'policyRequestStatus',
  ApprovalId: 'policyApprovalId',
  OverrideId: 'policyOverrideId',
  ReplayOfApprovalId: 'policyReplayOfApprovalId',
  ReviewedByActorId: 'policyReviewedByActorId',
  ReviewedByActorRole: 'policyReviewedByActorRole',
  ReviewedAt: 'policyReviewedAt',
  AuditReferenceId: 'policyAuditReferenceId',
  NetworkEvidenceGrade: 'networkEvidenceGrade',
  NetworkRequestedPolicyAction: 'networkRequestedPolicyAction',
  NetworkMappedPolicyAction: 'networkMappedPolicyAction',
  NetworkPolicyMappingMode: 'networkPolicyMappingMode',
  NetworkAdapterActionAuthorized: 'networkAdapterActionAuthorized',
  NetworkEnforcementCommandAuthorized: 'networkEnforcementCommandAuthorized',
} as const;

export const AgentPolicyPreviewDefaults = {
  TargetType: {
    App: 'app',
    Device: 'device',
    Url: 'url',
    Domain: 'domain',
    Site: 'site',
    Category: 'category',
    NetworkDomain: 'network-domain',
    Unknown: 'unknown',
  },
  Action: {
    Allow: 'allow',
    None: 'none',
    AskParent: 'ask-parent',
    WarnChild: 'warn-child',
    Monitor: 'monitor',
    Limit: 'limit',
    Block: 'block',
    ManualReview: 'manual-review',
  },
  EvidenceGrade: {
    A: 'A',
    B: 'B',
    C: 'C',
    D: 'D',
  },
  MappingMode: {
    ParentReview: 'parent-review',
    ObserveOnly: 'observe-only',
    DryRun: 'dry-run',
    ManualRequired: 'manual-required',
    AdapterUnavailable: 'adapter-unavailable',
  },
  SaveState: {
    PreviewRequired: 'preview-required',
    ReadyToSave: 'ready-to-save',
    Blocked: 'blocked',
  },
  ManualReviewState: {
    Required: 'required',
    NotRequired: 'not-required',
  },
  TargetState: {
    Supported: 'supported',
    Unsupported: 'unsupported',
    ManualRequired: 'manual-required',
    Offline: 'offline',
    Stale: 'stale',
  },
  SourceStatus: {
    Draft: 'draft',
    Preview: 'preview',
    Confirmed: 'confirmed',
    Queued: 'queued',
    Delivered: 'delivered',
    Acknowledged: 'acknowledged',
    Active: 'active',
    PartiallyActive: 'partially-active',
    Rejected: 'rejected',
    Superseded: 'superseded',
    RolledBack: 'rolled-back',
    Stale: 'stale',
    Expired: 'expired',
    ManualRequired: 'manual-required',
  },
  SourceSurface: {
    ParentPortal: 'parent-portal',
    ParentCompanion: 'parent-companion',
    AiPreview: 'ai-preview',
    DomainCache: 'domain-cache',
  },
  RequestOrigin: {
    Child: 'child',
    AssistantDraft: 'assistant-draft',
  },
  AssistantConfirmationState: {
    NotRequired: 'not-required',
    ParentConfirmationRequired: 'parent-confirmation-required',
    ParentConfirmed: 'parent-confirmed',
  },
  RequestStatus: {
    PreviewOnly: 'preview-only',
    PendingParentReview: 'pending-parent-review',
    Approved: 'approved',
    Denied: 'denied',
    Modified: 'modified',
    Expired: 'expired',
    ReplayRejected: 'replay-rejected',
  },
  HandoffState: {
    DisabledPreviewOnly: 'disabled-preview-only',
  },
  ValidationMessage: {
    DryRunPreviewOnlyHandoffRequired: 'Network policy preview fields require dry-run preview-only handoff',
  },
} as const;

const NullableTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);
const NullableSchemaVersionSchema = Schema.Union(NonEmptyStringSchema, Schema.Number, Schema.Null);
const NullableNumberSchema = Schema.Union(Schema.Number, Schema.Null);
const NullableBooleanSchema = Schema.Union(Schema.Boolean, Schema.Null);
const FalseOrNullSchema = Schema.Union(Schema.Literal(false), Schema.Null);

export const AgentPolicyPreviewTargetTypeSchema = withParser(
  Schema.Literal(
    AgentPolicyPreviewDefaults.TargetType.App,
    AgentPolicyPreviewDefaults.TargetType.Device,
    AgentPolicyPreviewDefaults.TargetType.Url,
    AgentPolicyPreviewDefaults.TargetType.Domain,
    AgentPolicyPreviewDefaults.TargetType.Site,
    AgentPolicyPreviewDefaults.TargetType.Category,
    AgentPolicyPreviewDefaults.TargetType.NetworkDomain,
    AgentPolicyPreviewDefaults.TargetType.Unknown
  )
);
export const AgentPolicyPreviewActionSchema = withParser(
  Schema.Literal(
    AgentPolicyPreviewDefaults.Action.Allow,
    AgentPolicyPreviewDefaults.Action.None,
    AgentPolicyPreviewDefaults.Action.AskParent,
    AgentPolicyPreviewDefaults.Action.WarnChild,
    AgentPolicyPreviewDefaults.Action.Monitor,
    AgentPolicyPreviewDefaults.Action.Limit,
    AgentPolicyPreviewDefaults.Action.Block,
    AgentPolicyPreviewDefaults.Action.ManualReview
  )
);
export const AgentPolicyPreviewNetworkPolicyMappingModeSchema = withParser(
  Schema.Literal(
    AgentPolicyPreviewDefaults.MappingMode.ParentReview,
    AgentPolicyPreviewDefaults.MappingMode.ObserveOnly,
    AgentPolicyPreviewDefaults.MappingMode.DryRun,
    AgentPolicyPreviewDefaults.MappingMode.ManualRequired,
    AgentPolicyPreviewDefaults.MappingMode.AdapterUnavailable
  )
);
export const AgentPolicyPreviewHandoffStateSchema = withParser(
  Schema.Literal(AgentPolicyPreviewDefaults.HandoffState.DisabledPreviewOnly)
);
export const AgentPolicyPreviewSaveStateSchema = withParser(
  Schema.Literal(
    AgentPolicyPreviewDefaults.SaveState.PreviewRequired,
    AgentPolicyPreviewDefaults.SaveState.ReadyToSave,
    AgentPolicyPreviewDefaults.SaveState.Blocked
  )
);
export const AgentPolicyPreviewManualReviewStateSchema = withParser(
  Schema.Literal(
    AgentPolicyPreviewDefaults.ManualReviewState.Required,
    AgentPolicyPreviewDefaults.ManualReviewState.NotRequired
  )
);
export const AgentPolicyPreviewTargetStateSchema = withParser(
  Schema.Literal(
    AgentPolicyPreviewDefaults.TargetState.Supported,
    AgentPolicyPreviewDefaults.TargetState.Unsupported,
    AgentPolicyPreviewDefaults.TargetState.ManualRequired,
    AgentPolicyPreviewDefaults.TargetState.Offline,
    AgentPolicyPreviewDefaults.TargetState.Stale
  )
);
export const AgentPolicyPreviewSourceStatusSchema = withParser(
  Schema.Literal(
    AgentPolicyPreviewDefaults.SourceStatus.Draft,
    AgentPolicyPreviewDefaults.SourceStatus.Preview,
    AgentPolicyPreviewDefaults.SourceStatus.Confirmed,
    AgentPolicyPreviewDefaults.SourceStatus.Queued,
    AgentPolicyPreviewDefaults.SourceStatus.Delivered,
    AgentPolicyPreviewDefaults.SourceStatus.Acknowledged,
    AgentPolicyPreviewDefaults.SourceStatus.Active,
    AgentPolicyPreviewDefaults.SourceStatus.PartiallyActive,
    AgentPolicyPreviewDefaults.SourceStatus.Rejected,
    AgentPolicyPreviewDefaults.SourceStatus.Superseded,
    AgentPolicyPreviewDefaults.SourceStatus.RolledBack,
    AgentPolicyPreviewDefaults.SourceStatus.Stale,
    AgentPolicyPreviewDefaults.SourceStatus.Expired,
    AgentPolicyPreviewDefaults.SourceStatus.ManualRequired
  )
);
export const AgentPolicyPreviewSourceSurfaceSchema = withParser(
  Schema.Literal(
    AgentPolicyPreviewDefaults.SourceSurface.ParentPortal,
    AgentPolicyPreviewDefaults.SourceSurface.ParentCompanion,
    AgentPolicyPreviewDefaults.SourceSurface.AiPreview,
    AgentPolicyPreviewDefaults.SourceSurface.DomainCache
  )
);
export const AgentPolicyPreviewRequestOriginSchema = withParser(
  Schema.Literal(
    AgentPolicyPreviewDefaults.RequestOrigin.Child,
    AgentPolicyPreviewDefaults.RequestOrigin.AssistantDraft
  )
);
export const AgentPolicyPreviewAssistantConfirmationStateSchema = withParser(
  Schema.Literal(
    AgentPolicyPreviewDefaults.AssistantConfirmationState.NotRequired,
    AgentPolicyPreviewDefaults.AssistantConfirmationState.ParentConfirmationRequired,
    AgentPolicyPreviewDefaults.AssistantConfirmationState.ParentConfirmed
  )
);
export const AgentPolicyPreviewRequestStatusSchema = withParser(
  Schema.Literal(
    AgentPolicyPreviewDefaults.RequestStatus.PreviewOnly,
    AgentPolicyPreviewDefaults.RequestStatus.PendingParentReview,
    AgentPolicyPreviewDefaults.RequestStatus.Approved,
    AgentPolicyPreviewDefaults.RequestStatus.Denied,
    AgentPolicyPreviewDefaults.RequestStatus.Modified,
    AgentPolicyPreviewDefaults.RequestStatus.Expired,
    AgentPolicyPreviewDefaults.RequestStatus.ReplayRejected
  )
);

const NullablePolicyTargetTypeSchema = Schema.Union(AgentPolicyPreviewTargetTypeSchema, Schema.Null);
const NullablePolicyActionSchema = Schema.Union(AgentPolicyPreviewActionSchema, Schema.Null);
const NullableNetworkEvidenceGradeSchema = Schema.Union(ActivityNetworkEvidenceGradeSchema, Schema.Null);
const NullableNetworkPolicyMappingModeSchema = Schema.Union(
  AgentPolicyPreviewNetworkPolicyMappingModeSchema,
  Schema.Null
);
const NullablePolicyHandoffStateSchema = Schema.Union(AgentPolicyPreviewHandoffStateSchema, Schema.Null);
const NullablePolicyPreviewSaveStateSchema = Schema.Union(AgentPolicyPreviewSaveStateSchema, Schema.Null);
const NullablePolicyPreviewManualReviewStateSchema = Schema.Union(
  AgentPolicyPreviewManualReviewStateSchema,
  Schema.Null
);
const NullablePolicyPreviewTargetStateSchema = Schema.Union(AgentPolicyPreviewTargetStateSchema, Schema.Null);
const NullablePolicySourceStatusSchema = Schema.Union(AgentPolicyPreviewSourceStatusSchema, Schema.Null);
const NullablePolicySourceSurfaceSchema = Schema.Union(AgentPolicyPreviewSourceSurfaceSchema, Schema.Null);
const NullablePolicyRequestOriginSchema = Schema.Union(AgentPolicyPreviewRequestOriginSchema, Schema.Null);
const NullablePolicyAssistantConfirmationStateSchema = Schema.Union(
  AgentPolicyPreviewAssistantConfirmationStateSchema,
  Schema.Null
);
const NullablePolicyRequestStatusSchema = Schema.Union(AgentPolicyPreviewRequestStatusSchema, Schema.Null);

export const PortalPolicyPreviewReadModelSchema = withParser(
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
          readModel.enforcementHandoffState === AgentPolicyPreviewDefaults.HandoffState.DisabledPreviewOnly) ||
        AgentPolicyPreviewDefaults.ValidationMessage.DryRunPreviewOnlyHandoffRequired
    )
  )
);

export type PortalPolicyPreviewReadModel = Infer<typeof PortalPolicyPreviewReadModelSchema>;

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
