/* thin adapter over Rust-owned generated policy compiler contracts */

import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import { ChildProfileIdSchema, ParentDeviceIdSchema } from './family-reference-primitives';
import {
  GeneratedPolicyCompilerCapabilityStateValues,
  GeneratedPolicyCompilerDomainValues,
  GeneratedPolicyCompilerNoClaimLabelValues,
  GeneratedPolicyCompilerRuleStatusValues,
  GeneratedPolicyCompilerSourceStatusValues,
  GeneratedPolicyCompilerTargetKindValues,
} from './generated-policy-control-helpers-contracts';
import { literalRecordFromValues, literalSchema, parsedLiteralRecord } from './policy-literal-contracts';
import { PolicyAuditReferenceIdSchema } from './policy-authority';
import {
  PolicyActionSchema,
  PolicyReasonCodeSchema,
  PolicyRuleIdSchema,
  PolicyScheduleIdSchema,
  PolicyScheduleTimeBudgetSchema,
} from './policy-contracts';

export const PolicyCompiledArtifactIdSchema = brandedNonEmptyStringSchema('PolicyCompiledArtifactId');
export const PolicyCompilerSchemaVersionSchema = withParser(Schema.Number.pipe(Schema.int(), Schema.positive()));
export const PolicyCompilerHouseholdIdSchema = brandedNonEmptyStringSchema('PolicyCompilerHouseholdId');
export const PolicyCompilerPolicyVersionSchema = brandedNonEmptyStringSchema('PolicyCompilerPolicyVersion');
export const PolicyCompilerSourceDocumentIdSchema = brandedNonEmptyStringSchema('PolicyCompilerSourceDocumentId');
export const PolicyCompilerTargetReferenceIdSchema = brandedNonEmptyStringSchema('PolicyCompilerTargetReferenceId');

export const PolicyCompilerDomainLiteral = literalRecordFromValues(GeneratedPolicyCompilerDomainValues);

export const PolicyCompilerRuleStatusLiteral = literalRecordFromValues(GeneratedPolicyCompilerRuleStatusValues);

export const PolicyCompilerCapabilityStateLiteral = literalRecordFromValues(
  GeneratedPolicyCompilerCapabilityStateValues
);

export const PolicyCompilerSourceStatusLiteral = literalRecordFromValues(GeneratedPolicyCompilerSourceStatusValues);

export const PolicyCompilerTargetKindLiteral = literalRecordFromValues(GeneratedPolicyCompilerTargetKindValues);

export const PolicyCompilerNoClaimLabelLiteral = literalRecordFromValues(GeneratedPolicyCompilerNoClaimLabelValues);

export const PolicyCompilerDomainSchema = literalSchema(PolicyCompilerDomainLiteral);
export const PolicyCompilerRuleStatusSchema = literalSchema(PolicyCompilerRuleStatusLiteral);
export const PolicyCompilerCapabilityStateSchema = literalSchema(PolicyCompilerCapabilityStateLiteral);
export const PolicyCompilerSourceStatusSchema = literalSchema(PolicyCompilerSourceStatusLiteral);
export const PolicyCompilerTargetKindSchema = literalSchema(PolicyCompilerTargetKindLiteral);
export const PolicyCompilerNoClaimLabelSchema = literalSchema(PolicyCompilerNoClaimLabelLiteral);

export const PolicyCompilerTargetSchema = withParser(
  Schema.Struct({
    kind: PolicyCompilerTargetKindSchema,
    referenceId: PolicyCompilerTargetReferenceIdSchema,
  })
);

const PolicyCompilerRuleBaseSchema = Schema.Struct({
  ruleId: PolicyRuleIdSchema,
  target: PolicyCompilerTargetSchema,
  action: PolicyActionSchema,
  scheduleId: Schema.Union(PolicyScheduleIdSchema, Schema.Null),
  capabilityState: PolicyCompilerCapabilityStateSchema,
  status: PolicyCompilerRuleStatusSchema,
  reasonCode: Schema.Union(PolicyReasonCodeSchema, Schema.Null),
});

export const PolicyCompilerRuleSchema = withParser(PolicyCompilerRuleBaseSchema);

export const PolicyCompilerSupportMatrixRowSchema = withParser(
  Schema.Struct({
    targetKind: PolicyCompilerTargetKindSchema,
    capabilityState: PolicyCompilerCapabilityStateSchema,
  })
);

export const PolicyCompilerSupportMatrixSchema = withParser(
  Schema.Struct({
    domain: PolicyCompilerDomainSchema,
    rows: Schema.Array(PolicyCompilerSupportMatrixRowSchema),
  })
);

export const PolicyCompilerDeliveryTargetSchema = withParser(
  Schema.Struct({
    childProfileIds: Schema.Array(ChildProfileIdSchema),
    deviceIds: Schema.Array(ParentDeviceIdSchema),
    domain: PolicyCompilerDomainSchema,
  })
);

export const PolicyCompilerEvidenceCustodyRequirementsSchema = withParser(
  Schema.Struct({
    exportAllowed: Schema.Boolean,
    deleteAllowed: Schema.Boolean,
    syncAllowed: Schema.Boolean,
  })
);

export const PolicyCompilerRollbackRefSchema = withParser(
  Schema.Struct({
    householdId: PolicyCompilerHouseholdIdSchema,
    rolledBackDocumentId: PolicyCompilerSourceDocumentIdSchema,
    rolledBackPolicyVersion: PolicyCompilerPolicyVersionSchema,
    restoredDocumentId: PolicyCompilerSourceDocumentIdSchema,
    restoredPolicyVersion: PolicyCompilerPolicyVersionSchema,
  })
);

export const PolicyCompilerScheduleWindowSchema = withParser(
  Schema.Struct({
    scheduleId: PolicyScheduleIdSchema,
    timeZone: brandedNonEmptyStringSchema('PolicyCompilerTimeZone'),
    startsAt: brandedNonEmptyStringSchema('PolicyCompilerScheduleStartsAt'),
    endsAt: brandedNonEmptyStringSchema('PolicyCompilerScheduleEndsAt'),
    timeBudget: PolicyScheduleTimeBudgetSchema,
  })
);

export const PolicyCompiledArtifactSchema = withParser(
  Schema.Struct({
    compiledArtifactId: PolicyCompiledArtifactIdSchema,
    compilerSchemaVersion: PolicyCompilerSchemaVersionSchema,
    householdId: PolicyCompilerHouseholdIdSchema,
    sourcePolicyVersion: PolicyCompilerPolicyVersionSchema,
    consumerPolicyVersion: PolicyCompilerPolicyVersionSchema,
    sourceDocumentId: PolicyCompilerSourceDocumentIdSchema,
    sourceStatus: PolicyCompilerSourceStatusSchema,
    domain: PolicyCompilerDomainSchema,
    deliveryTarget: PolicyCompilerDeliveryTargetSchema,
    supportMatrix: PolicyCompilerSupportMatrixSchema,
    evidenceCustodyRequirements: PolicyCompilerEvidenceCustodyRequirementsSchema,
    noClaimLabels: Schema.Array(PolicyCompilerNoClaimLabelSchema),
    auditReferenceIds: Schema.Array(PolicyAuditReferenceIdSchema),
    supersededByPolicyVersion: Schema.Union(PolicyCompilerPolicyVersionSchema, Schema.Null),
    rollbackRef: Schema.Union(PolicyCompilerRollbackRefSchema, Schema.Null),
    schedules: Schema.Array(PolicyCompilerScheduleWindowSchema),
    rules: Schema.Array(PolicyCompilerRuleSchema),
  })
);

export type PolicyCompiledArtifactId = typeof PolicyCompiledArtifactIdSchema.Type;
export type PolicyCompilerHouseholdId = typeof PolicyCompilerHouseholdIdSchema.Type;
export type PolicyCompilerPolicyVersion = typeof PolicyCompilerPolicyVersionSchema.Type;
export type PolicyCompilerSourceDocumentId = typeof PolicyCompilerSourceDocumentIdSchema.Type;
export type PolicyCompilerDomain = Infer<typeof PolicyCompilerDomainSchema>;
export type PolicyCompilerRuleStatus = Infer<typeof PolicyCompilerRuleStatusSchema>;
export type PolicyCompilerCapabilityState = Infer<typeof PolicyCompilerCapabilityStateSchema>;
export type PolicyCompilerSourceStatus = Infer<typeof PolicyCompilerSourceStatusSchema>;
export type PolicyCompilerTargetKind = Infer<typeof PolicyCompilerTargetKindSchema>;
export type PolicyCompilerNoClaimLabel = Infer<typeof PolicyCompilerNoClaimLabelSchema>;
export type PolicyCompilerTarget = Infer<typeof PolicyCompilerTargetSchema>;
export type PolicyCompilerRule = Infer<typeof PolicyCompilerRuleSchema>;
export type PolicyCompilerSupportMatrixRow = Infer<typeof PolicyCompilerSupportMatrixRowSchema>;
export type PolicyCompilerSupportMatrix = Infer<typeof PolicyCompilerSupportMatrixSchema>;
export type PolicyCompilerDeliveryTarget = Infer<typeof PolicyCompilerDeliveryTargetSchema>;
export type PolicyCompilerEvidenceCustodyRequirements = Infer<typeof PolicyCompilerEvidenceCustodyRequirementsSchema>;
export type PolicyCompilerRollbackRef = Infer<typeof PolicyCompilerRollbackRefSchema>;
export type PolicyCompilerScheduleWindow = Infer<typeof PolicyCompilerScheduleWindowSchema>;
export type PolicyCompiledArtifact = Infer<typeof PolicyCompiledArtifactSchema>;

export const PolicyCompilerDomain = parsedLiteralRecord(PolicyCompilerDomainLiteral, (value) =>
  PolicyCompilerDomainSchema.parse(value)
);

export const PolicyCompilerRuleStatus = parsedLiteralRecord(PolicyCompilerRuleStatusLiteral, (value) =>
  PolicyCompilerRuleStatusSchema.parse(value)
);

export const PolicyCompilerCapabilityState = parsedLiteralRecord(PolicyCompilerCapabilityStateLiteral, (value) =>
  PolicyCompilerCapabilityStateSchema.parse(value)
);

export const PolicyCompilerNoClaimLabel = parsedLiteralRecord(PolicyCompilerNoClaimLabelLiteral, (value) =>
  PolicyCompilerNoClaimLabelSchema.parse(value)
);

export function parsePolicyCompiledArtifact(input: unknown): PolicyCompiledArtifact {
  return PolicyCompiledArtifactSchema.parse(input);
}
