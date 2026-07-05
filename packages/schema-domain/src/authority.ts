import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import {
  ChildProfileReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
} from './family-references';
import { literalSchema, parsedLiteralRecord } from './literal-contracts';
import {
  PermissionRequestIdSchema,
  PolicyActionSchema,
  PolicyDecisionSchema,
  PolicyScheduleBoundarySchema,
  PolicyTargetSchema,
  PolicyTimestampSchema,
} from './policy';
import {
  resolveGeneratedPolicyApprovalLifecycle,
  resolveGeneratedPolicyAuthority,
} from './generated-policy-control-helpers';

export const PolicyAuthoritySourceLiteral = {
  ParentPolicy: 'parent-policy',
  LocalAiResult: 'local-ai-result',
  TrackingSignal: 'tracking-signal',
  ActivityEvidence: 'activity-evidence',
} as const;

const PolicyAuthorityStateLiteral = {
  Authorized: 'authorized',
  EvidenceOnly: 'evidence-only',
  DryRun: 'dry-run',
} as const;

export const PolicyApprovalIdSchema = brandedNonEmptyStringSchema('PolicyApprovalId');
export const PolicyOverrideIdSchema = brandedNonEmptyStringSchema('PolicyOverrideId');
export const PolicyAuditReferenceIdSchema = brandedNonEmptyStringSchema('PolicyAuditReferenceId');

export const PolicyApprovalOriginLiteral = {
  ChildRequest: 'child-request',
  AssistantDraft: 'assistant-draft',
} as const;

export const PolicyApprovalKindLiteral = {
  AskParent: 'ask-parent',
  TemporaryOverride: 'temporary-override',
  BonusTime: 'bonus-time',
} as const;

export const PolicyApprovalStateLiteral = {
  Pending: 'pending',
  Approved: 'approved',
  Denied: 'denied',
  Modified: 'modified',
  ExpiredRequest: 'expired-request',
  ReplayRejected: 'replay-rejected',
  PreviewOnly: 'preview-only',
} as const;

export const PolicyOverrideTypeLiteral = {
  TemporaryAllow: 'temporary-allow',
  TemporaryBlock: 'temporary-block',
  BonusTime: 'bonus-time',
} as const;

export const PolicyOverrideStateLiteral = {
  Active: 'active',
  Expired: 'expired',
  Revoked: 'revoked',
} as const;

export const PolicyAuthoritySourceSchema = literalSchema(PolicyAuthoritySourceLiteral);
export const PolicyAuthorityStateSchema = literalSchema(PolicyAuthorityStateLiteral);
export const PolicyApprovalOriginSchema = literalSchema(PolicyApprovalOriginLiteral);
export const PolicyApprovalKindSchema = literalSchema(PolicyApprovalKindLiteral);
export const PolicyApprovalStateSchema = literalSchema(PolicyApprovalStateLiteral);
export const PolicyOverrideTypeSchema = literalSchema(PolicyOverrideTypeLiteral);
export const PolicyOverrideStateSchema = literalSchema(PolicyOverrideStateLiteral);

export const PolicyAuthorityRequestSchema = withParser(
  Schema.Struct({
    source: PolicyAuthoritySourceSchema,
    decision: PolicyDecisionSchema,
  })
);

export const PolicyAuthorityDecisionSchema = withParser(
  Schema.Struct({
    source: PolicyAuthoritySourceSchema,
    state: PolicyAuthorityStateSchema,
    decision: PolicyDecisionSchema,
  })
);

export const PolicyApprovalRequestSchema = withParser(
  Schema.Struct({
    approvalId: PolicyApprovalIdSchema,
    permissionRequestId: PermissionRequestIdSchema,
    origin: PolicyApprovalOriginSchema,
    kind: PolicyApprovalKindSchema,
    childProfile: ChildProfileReferenceSchema,
    device: ParentDeviceReferenceSchema,
    requestedTarget: PolicyTargetSchema,
    requestedAction: PolicyActionSchema,
    requestedAt: PolicyTimestampSchema,
    expiresAt: PolicyTimestampSchema,
    requestedBonusTimeMinutes: Schema.Union(Schema.Number, Schema.Null),
    scheduleBoundary: Schema.Union(PolicyScheduleBoundarySchema, Schema.Null),
  })
);

export const PolicyOverrideGrantSchema = withParser(
  Schema.Struct({
    overrideId: PolicyOverrideIdSchema,
    overrideType: PolicyOverrideTypeSchema,
    state: PolicyOverrideStateSchema,
    action: PolicyActionSchema,
    effectiveFrom: PolicyTimestampSchema,
    effectiveUntil: PolicyTimestampSchema,
    bonusTimeMinutes: Schema.Union(Schema.Number, Schema.Null),
  })
);

export const PolicyApprovalResolutionSchema = withParser(
  Schema.Struct({
    approval: PolicyApprovalRequestSchema,
    state: PolicyApprovalStateSchema,
    evaluatedAt: PolicyTimestampSchema,
    reviewedBy: Schema.Union(ParentActorReferenceSchema, Schema.Null),
    reviewedAt: Schema.Union(PolicyTimestampSchema, Schema.Null),
    auditReferenceId: Schema.Union(PolicyAuditReferenceIdSchema, Schema.Null),
    override: Schema.Union(PolicyOverrideGrantSchema, Schema.Null),
    replayOfApprovalId: Schema.Union(PolicyApprovalIdSchema, Schema.Null),
  })
);

export type PolicyAuthoritySource = Infer<typeof PolicyAuthoritySourceSchema>;
export type PolicyAuthorityState = Infer<typeof PolicyAuthorityStateSchema>;
export type PolicyAuthorityRequest = Infer<typeof PolicyAuthorityRequestSchema>;
export type PolicyAuthorityDecision = Infer<typeof PolicyAuthorityDecisionSchema>;
export type PolicyApprovalId = typeof PolicyApprovalIdSchema.Type;
export type PolicyOverrideId = typeof PolicyOverrideIdSchema.Type;
export type PolicyAuditReferenceId = typeof PolicyAuditReferenceIdSchema.Type;
export type PolicyApprovalOrigin = Infer<typeof PolicyApprovalOriginSchema>;
export type PolicyApprovalKind = Infer<typeof PolicyApprovalKindSchema>;
export type PolicyApprovalState = Infer<typeof PolicyApprovalStateSchema>;
export type PolicyOverrideType = Infer<typeof PolicyOverrideTypeSchema>;
export type PolicyOverrideState = Infer<typeof PolicyOverrideStateSchema>;
export type PolicyApprovalRequest = Infer<typeof PolicyApprovalRequestSchema>;
export type PolicyOverrideGrant = Infer<typeof PolicyOverrideGrantSchema>;
export type PolicyApprovalResolution = Infer<typeof PolicyApprovalResolutionSchema>;

export const PolicyAuthoritySource = parsedLiteralRecord(PolicyAuthoritySourceLiteral, (value) =>
  PolicyAuthoritySourceSchema.parse(value)
);

export const PolicyAuthorityState = parsedLiteralRecord(PolicyAuthorityStateLiteral, (value) =>
  PolicyAuthorityStateSchema.parse(value)
);

export const PolicyApprovalOrigin = parsedLiteralRecord(PolicyApprovalOriginLiteral, (value) =>
  PolicyApprovalOriginSchema.parse(value)
);

export const PolicyApprovalKind = parsedLiteralRecord(PolicyApprovalKindLiteral, (value) =>
  PolicyApprovalKindSchema.parse(value)
);

export const PolicyApprovalState = parsedLiteralRecord(PolicyApprovalStateLiteral, (value) =>
  PolicyApprovalStateSchema.parse(value)
);

export const PolicyOverrideType = parsedLiteralRecord(PolicyOverrideTypeLiteral, (value) =>
  PolicyOverrideTypeSchema.parse(value)
);

export const PolicyOverrideState = parsedLiteralRecord(PolicyOverrideStateLiteral, (value) =>
  PolicyOverrideStateSchema.parse(value)
);

export function resolvePolicyAuthority(input: PolicyAuthorityRequest): PolicyAuthorityDecision {
  const request = PolicyAuthorityRequestSchema.parse(input);
  return PolicyAuthorityDecisionSchema.parse(resolveGeneratedPolicyAuthority(request));
}

export function resolvePolicyApprovalLifecycle(input: unknown): PolicyApprovalResolution {
  const resolution = PolicyApprovalResolutionSchema.parse(input);
  return PolicyApprovalResolutionSchema.parse(resolveGeneratedPolicyApprovalLifecycle(resolution));
}
