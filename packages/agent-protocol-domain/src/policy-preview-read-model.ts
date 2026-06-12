import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentProtocolDefaults, type AgentProtocolLogFields } from './contracts';

const NullableTextSchema = Schema.Union(Schema.String, Schema.Null);
const NullableSchemaVersionSchema = Schema.Union(Schema.String, Schema.Number, Schema.Null);
const NullableNumberSchema = Schema.Union(Schema.Number, Schema.Null);
const NullableBooleanSchema = Schema.Union(Schema.Boolean, Schema.Null);
const FalseOrNullSchema = Schema.Union(Schema.Literal(false), Schema.Null);
const PolicyPreviewDefaults = AgentProtocolDefaults.PolicyPreview;
const NullablePolicyTargetTypeSchema = Schema.Union(
  Schema.Literal(
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
    schemaVersion: valueOrNull(payload[AgentProtocolDefaults.Field.SchemaVersion]),
    generatedAt: valueOrNull(payload[AgentProtocolDefaults.Field.GeneratedAt]),
    custody: valueOrNull(payload[AgentProtocolDefaults.Field.Custody]),
    limit: valueOrNull(payload[AgentProtocolDefaults.Field.Limit]),
    returned: payload[AgentProtocolDefaults.Field.Returned],
    capabilityStatus: valueOrNull(payload[AgentProtocolDefaults.Field.CapabilityStatus]),
    previewId: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyPreviewId]),
    latestEventId: valueOrNull(payload[AgentProtocolDefaults.Field.LatestEventId]),
    latestObservedAt: valueOrNull(payload[AgentProtocolDefaults.Field.LatestObservedAt]),
    targetId: valueOrNull(payload[AgentProtocolDefaults.Field.TargetId]),
    targetType: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyTargetType]),
    targetValue: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyTargetValue]),
    evidenceReferenceCount: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyEvidenceReferenceCount]),
    parentRuleContextReferenceCount: valueOrNull(
      payload[AgentProtocolDefaults.Field.PolicyParentRuleContextReferenceCount]
    ),
    parentRuleContextRefIds: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyParentRuleContextRefIds]),
    decisionId: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyDecisionId]),
    decisionAction: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyAction]),
    reasonCodes: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyReasonCodes]),
    ruleIds: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyRuleIds]),
    localAiResultId: valueOrNull(payload[AgentProtocolDefaults.Field.LocalAiResultId]),
    dryRun: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyDryRun]),
    enforcementHandoffState: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyHandoffState]),
    networkEvidenceGrade: valueOrNull(payload[AgentProtocolDefaults.Field.NetworkEvidenceGrade]),
    networkRequestedPolicyAction: valueOrNull(payload[AgentProtocolDefaults.Field.NetworkRequestedPolicyAction]),
    networkMappedPolicyAction: valueOrNull(payload[AgentProtocolDefaults.Field.NetworkMappedPolicyAction]),
    networkPolicyMappingMode: valueOrNull(payload[AgentProtocolDefaults.Field.NetworkPolicyMappingMode]),
    networkAdapterActionAuthorized: valueOrNull(payload[AgentProtocolDefaults.Field.NetworkAdapterActionAuthorized]),
    networkEnforcementCommandAuthorized: valueOrNull(
      payload[AgentProtocolDefaults.Field.NetworkEnforcementCommandAuthorized]
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
