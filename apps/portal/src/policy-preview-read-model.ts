import { AgentProtocolDefaults, type AgentProtocolLogFields } from '@ocentra-parent/agent-protocol-domain/contracts';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NullableTextSchema = Schema.Union(Schema.String, Schema.Null);
const NullableSchemaVersionSchema = Schema.Union(Schema.String, Schema.Number, Schema.Null);
const NullableNumberSchema = Schema.Union(Schema.Number, Schema.Null);
const NullableBooleanSchema = Schema.Union(Schema.Boolean, Schema.Null);

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
    targetType: NullableTextSchema,
    targetValue: NullableTextSchema,
    evidenceReferenceCount: NullableNumberSchema,
    parentRuleContextReferenceCount: NullableNumberSchema,
    parentRuleContextRefIds: NullableTextSchema,
    decisionId: NullableTextSchema,
    decisionAction: NullableTextSchema,
    reasonCodes: NullableTextSchema,
    ruleIds: NullableTextSchema,
    localAiResultId: NullableTextSchema,
    dryRun: NullableBooleanSchema,
    enforcementHandoffState: NullableTextSchema,
  })
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
  });

  if (!parsed.success) {
    return null;
  }
  return parsed.data;
}

function valueOrNull(value: unknown): unknown {
  return value === undefined ? null : value;
}
