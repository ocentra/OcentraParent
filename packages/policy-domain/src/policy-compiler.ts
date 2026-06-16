import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ChildProfileIdSchema, ParentDeviceIdSchema } from '@ocentra-parent/family-domain/reference-primitives';
import { PolicyAuditReferenceIdSchema } from './authority';
import {
  PolicyActionSchema,
  PolicyReasonCodeSchema,
  PolicyRuleIdSchema,
  PolicyScheduleIdSchema,
  PolicyScheduleTimeBudgetSchema,
} from './policy';

export const PolicyCompiledArtifactIdSchema = brandedNonEmptyStringSchema('PolicyCompiledArtifactId');
export const PolicyCompilerSchemaVersionSchema = withParser(Schema.Number.pipe(Schema.int(), Schema.positive()));
export const PolicyCompilerHouseholdIdSchema = brandedNonEmptyStringSchema('PolicyCompilerHouseholdId');
export const PolicyCompilerPolicyVersionSchema = brandedNonEmptyStringSchema('PolicyCompilerPolicyVersion');
export const PolicyCompilerSourceDocumentIdSchema = brandedNonEmptyStringSchema('PolicyCompilerSourceDocumentId');
export const PolicyCompilerTargetReferenceIdSchema = brandedNonEmptyStringSchema('PolicyCompilerTargetReferenceId');

export const PolicyCompilerDomainLiteral = {
  AppGame: 'app-game',
  Browser: 'browser',
  Network: 'network',
  Tracking: 'tracking',
  Screen: 'screen',
  Ai: 'ai',
  Enforcement: 'enforcement',
  NotificationAskParent: 'notification-ask-parent',
} as const;

export const PolicyCompilerRuleStatusLiteral = {
  Ready: 'ready',
  ManualRequired: 'manual-required',
  Unsupported: 'unsupported',
} as const;

export const PolicyCompilerCapabilityStateLiteral = {
  Supported: 'supported',
  ManualRequired: 'manual-required',
  Unsupported: 'unsupported',
} as const;

export const PolicyCompilerSourceStatusLiteral = {
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
} as const;

export const PolicyCompilerTargetKindLiteral = {
  ChildProfile: 'child-profile',
  Device: 'device',
  App: 'app',
  Site: 'site',
  Category: 'category',
  Resource: 'resource',
} as const;

export const PolicyCompilerNoClaimLabelLiteral = {
  CompiledArtifactNotSourceTruth: 'compiled-artifact-not-source-truth',
  RuntimeMutationNotClaimed: 'runtime-mutation-not-claimed',
  EnforcementNotClaimed: 'enforcement-not-claimed',
  UiDeliveryNotClaimed: 'ui-delivery-not-claimed',
  PlatformSupportNotClaimed: 'platform-support-not-claimed',
} as const;

export const PolicyCompilerDomainSchema = withParser(
  Schema.Literal(...Object.values(PolicyCompilerDomainLiteral))
);

export const PolicyCompilerRuleStatusSchema = withParser(
  Schema.Literal(...Object.values(PolicyCompilerRuleStatusLiteral))
);

export const PolicyCompilerCapabilityStateSchema = withParser(
  Schema.Literal(...Object.values(PolicyCompilerCapabilityStateLiteral))
);

export const PolicyCompilerSourceStatusSchema = withParser(
  Schema.Literal(...Object.values(PolicyCompilerSourceStatusLiteral))
);

export const PolicyCompilerTargetKindSchema = withParser(
  Schema.Literal(...Object.values(PolicyCompilerTargetKindLiteral))
);

export const PolicyCompilerNoClaimLabelSchema = withParser(
  Schema.Literal(...Object.values(PolicyCompilerNoClaimLabelLiteral))
);

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

type PolicyCompilerRuleCandidate = Infer<typeof PolicyCompilerRuleBaseSchema>;

export const PolicyCompilerRuleSchema = withParser(
  PolicyCompilerRuleBaseSchema.pipe(
    Schema.filter(
      (rule: PolicyCompilerRuleCandidate) =>
        (hasAlignedRuleCapabilityStateAndStatus(rule) && hasAlignedRuleStatusAndReasonCode(rule)) ||
        'Expected compiler rule capabilityState, status, and reasonCode to stay aligned'
    )
  )
);

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
  }).pipe(
    Schema.filter(
      (supportMatrix) =>
        hasExactlyOneSupportMatrixRowPerTargetKind(supportMatrix.rows) ||
        'Expected compiler support matrices to classify every target kind exactly once'
    )
  )
);

export const PolicyCompilerDeliveryTargetSchema = withParser(
  Schema.Struct({
    childProfileIds: Schema.Array(ChildProfileIdSchema),
    deviceIds: Schema.Array(ParentDeviceIdSchema),
    domain: PolicyCompilerDomainSchema,
  }).pipe(
    Schema.filter(
      (target) =>
        (target.childProfileIds.length > 0 && target.deviceIds.length > 0) ||
        'Expected compiler delivery targets to cite child profiles and devices'
    )
  )
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
    .pipe(
      Schema.filter(
        (artifact) =>
          hasExactlyRequiredNoClaimLabels(artifact.noClaimLabels) ||
          'Expected compiler artifacts to carry the full no-claim set exactly once'
      )
    )
    .pipe(
      Schema.filter(
        (artifact) =>
          hasUniqueAuditReferenceIds(artifact.auditReferenceIds) ||
          'Expected compiler artifacts to carry unique audit refs'
      )
    )
    .pipe(
      Schema.filter(
        (artifact) =>
          !(artifact.supersededByPolicyVersion !== null && artifact.rollbackRef !== null) ||
          'Expected compiler artifacts to keep supersede and rollback refs mutually exclusive'
      )
    )
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

export const PolicyCompilerDomain = Object.freeze(
  Object.fromEntries(
    Object.values(PolicyCompilerDomainLiteral).map((value) => [constantKey(value), PolicyCompilerDomainSchema.parse(value)])
  )
) as Readonly<Record<keyof typeof PolicyCompilerDomainLiteral, PolicyCompilerDomain>>;

export const PolicyCompilerRuleStatus = Object.freeze(
  Object.fromEntries(
    Object.values(PolicyCompilerRuleStatusLiteral).map((value) => [
      constantKey(value),
      PolicyCompilerRuleStatusSchema.parse(value),
    ])
  )
) as Readonly<Record<keyof typeof PolicyCompilerRuleStatusLiteral, PolicyCompilerRuleStatus>>;

export const PolicyCompilerCapabilityState = Object.freeze(
  Object.fromEntries(
    Object.values(PolicyCompilerCapabilityStateLiteral).map((value) => [
      constantKey(value),
      PolicyCompilerCapabilityStateSchema.parse(value),
    ])
  )
) as Readonly<
  Record<keyof typeof PolicyCompilerCapabilityStateLiteral, PolicyCompilerCapabilityState>
>;

export const PolicyCompilerNoClaimLabel = Object.freeze(
  Object.fromEntries(
    Object.values(PolicyCompilerNoClaimLabelLiteral).map((value) => [
      constantKey(value),
      PolicyCompilerNoClaimLabelSchema.parse(value),
    ])
  )
) as Readonly<Record<keyof typeof PolicyCompilerNoClaimLabelLiteral, PolicyCompilerNoClaimLabel>>;

export function parsePolicyCompiledArtifact(input: unknown): PolicyCompiledArtifact {
  return PolicyCompiledArtifactSchema.parse(input);
}

function hasExactlyRequiredNoClaimLabels(labels: readonly PolicyCompilerNoClaimLabel[]): boolean {
  const required = new Set(Object.values(PolicyCompilerNoClaimLabelLiteral));
  if (labels.length !== required.size) {
    return false;
  }

  for (const label of labels) {
    if (!required.delete(label)) {
      return false;
    }
  }

  return required.size === 0;
}

function hasAlignedRuleCapabilityStateAndStatus(rule: PolicyCompilerRuleCandidate): boolean {
  return rule.status === expectedRuleStatusForCapabilityState(rule.capabilityState);
}

function expectedRuleStatusForCapabilityState(
  capabilityState: PolicyCompilerCapabilityState
): PolicyCompilerRuleStatus {
  switch (capabilityState) {
    case PolicyCompilerCapabilityStateLiteral.Supported:
      return PolicyCompilerRuleStatusLiteral.Ready;
    case PolicyCompilerCapabilityStateLiteral.ManualRequired:
      return PolicyCompilerRuleStatusLiteral.ManualRequired;
    case PolicyCompilerCapabilityStateLiteral.Unsupported:
      return PolicyCompilerRuleStatusLiteral.Unsupported;
  }
}

function hasAlignedRuleStatusAndReasonCode(rule: PolicyCompilerRuleCandidate): boolean {
  return (
    (rule.status === PolicyCompilerRuleStatusLiteral.Ready && rule.reasonCode === null) ||
    ((rule.status === PolicyCompilerRuleStatusLiteral.ManualRequired ||
      rule.status === PolicyCompilerRuleStatusLiteral.Unsupported) &&
      rule.reasonCode !== null)
  );
}

function hasUniqueAuditReferenceIds(auditReferenceIds: readonly string[]): boolean {
  return auditReferenceIds.length > 0 && new Set(auditReferenceIds).size === auditReferenceIds.length;
}

function hasExactlyOneSupportMatrixRowPerTargetKind(
  rows: readonly PolicyCompilerSupportMatrixRow[]
): boolean {
  const required = new Set(Object.values(PolicyCompilerTargetKindLiteral));
  if (rows.length !== required.size) {
    return false;
  }

  for (const row of rows) {
    if (!required.delete(row.targetKind)) {
      return false;
    }
  }

  return required.size === 0;
}

function constantKey(value: string): string {
  return value
    .split('-')
    .map((segment) => segment.charAt(0).toUpperCase() + segment.slice(1))
    .join('');
}
