/* thin adapter over Rust-owned generated policy authority contracts */

import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import {
  GeneratedPolicyApprovalKindValues,
  GeneratedPolicyApprovalOriginValues,
  GeneratedPolicyApprovalStateValues,
  GeneratedPolicyAuthoritySourceValues,
  GeneratedPolicyAuthorityStateValues,
  GeneratedPolicyOverrideStateValues,
  GeneratedPolicyOverrideTypeValues,
} from './generated-policy-control-helpers-contracts';
import {
  ChildProfileReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
} from './family-references';
import { literalRecordFromValues, literalSchema, parsedLiteralRecord } from './policy-literal-contracts';
import {
  PermissionRequestIdSchema,
  PolicyActionSchema,
  PolicyDecisionSchema,
  PolicyScheduleBoundarySchema,
  PolicyTargetSchema,
  PolicyTimestampSchema,
} from './policy-contracts';

export const PolicyAuthoritySourceLiteral = literalRecordFromValues(GeneratedPolicyAuthoritySourceValues);

export const PolicyAuthorityStateLiteral = literalRecordFromValues(GeneratedPolicyAuthorityStateValues);

export const PolicyApprovalIdSchema = brandedNonEmptyStringSchema('PolicyApprovalId');
export const PolicyOverrideIdSchema = brandedNonEmptyStringSchema('PolicyOverrideId');
export const PolicyAuditReferenceIdSchema = brandedNonEmptyStringSchema('PolicyAuditReferenceId');

export const PolicyApprovalOriginLiteral = literalRecordFromValues(GeneratedPolicyApprovalOriginValues);

export const PolicyApprovalKindLiteral = literalRecordFromValues(GeneratedPolicyApprovalKindValues);

export const PolicyApprovalStateLiteral = literalRecordFromValues(GeneratedPolicyApprovalStateValues);

export const PolicyOverrideTypeLiteral = literalRecordFromValues(GeneratedPolicyOverrideTypeValues);

export const PolicyOverrideStateLiteral = literalRecordFromValues(GeneratedPolicyOverrideStateValues);

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
