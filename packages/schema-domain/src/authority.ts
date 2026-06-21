import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import {
  ChildProfileReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
} from './family-references';
import { literalSchema, parsedLiteralRecord } from './literal-contracts';
import {
  PermissionRequestIdSchema,
  PolicyAction,
  PolicyActionSchema,
  PolicyDecisionSchema,
  PolicyScheduleBoundarySchema,
  PolicyTargetSchema,
  PolicyTimestampSchema,
  parsePolicyScheduleBoundary,
} from './policy';

export const PolicyAuthoritySourceLiteral = {
  ParentPolicy: 'parent-policy',
  LocalAiResult: 'local-ai-result',
  TrackingSignal: 'tracking-signal',
  ActivityEvidence: 'activity-evidence',
} as const;

export const PolicyAuthorityStateLiteral = {
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

function assertAuthorityContract(condition: unknown, message: string): asserts condition {
  if (!condition) {
    throw new Error(message);
  }
}

function parseTimestampMillis(timestamp: string, fieldName: string): number {
  const millis = Date.parse(timestamp);
  assertAuthorityContract(!Number.isNaN(millis), `${fieldName} must be an ISO-8601 timestamp`);
  return millis;
}

function validatePolicyApprovalRequest(request: PolicyApprovalRequest): void {
  const requestedAt = parseTimestampMillis(request.requestedAt, 'approval.requestedAt');
  const expiresAt = parseTimestampMillis(request.expiresAt, 'approval.expiresAt');

  assertAuthorityContract(expiresAt > requestedAt, 'approval.expiresAt must be after approval.requestedAt');

  if (request.scheduleBoundary !== null) {
    parsePolicyScheduleBoundary(request.scheduleBoundary);
  }

  if (request.kind === PolicyApprovalKind.BonusTime) {
    assertAuthorityContract(
      request.requestedBonusTimeMinutes !== null && request.requestedBonusTimeMinutes > 0,
      'bonus-time requests must include a positive requestedBonusTimeMinutes value'
    );
    assertAuthorityContract(
      request.scheduleBoundary !== null,
      'bonus-time requests must include scheduleBoundary details'
    );
    assertAuthorityContract(
      request.scheduleBoundary.timeBudget !== null,
      'bonus-time requests must include scheduleBoundary.timeBudget details'
    );
  } else {
    assertAuthorityContract(
      request.requestedBonusTimeMinutes === null,
      'only bonus-time requests may include requestedBonusTimeMinutes'
    );
  }
}

function validatePolicyOverrideGrant(
  grant: PolicyOverrideGrant,
  approval: PolicyApprovalRequest,
  evaluatedAt: number
): void {
  const effectiveFrom = parseTimestampMillis(grant.effectiveFrom, 'override.effectiveFrom');
  const effectiveUntil = parseTimestampMillis(grant.effectiveUntil, 'override.effectiveUntil');

  assertAuthorityContract(
    effectiveUntil > effectiveFrom,
    'override.effectiveUntil must be after override.effectiveFrom'
  );

  switch (grant.overrideType) {
    case PolicyOverrideType.TemporaryAllow:
      assertAuthorityContract(grant.action === PolicyAction.Allow, 'temporary-allow overrides must resolve to allow');
      assertAuthorityContract(
        grant.bonusTimeMinutes === null,
        'temporary-allow overrides cannot carry bonusTimeMinutes'
      );
      break;
    case PolicyOverrideType.TemporaryBlock:
      assertAuthorityContract(grant.action === PolicyAction.Block, 'temporary-block overrides must resolve to block');
      assertAuthorityContract(
        grant.bonusTimeMinutes === null,
        'temporary-block overrides cannot carry bonusTimeMinutes'
      );
      break;
    case PolicyOverrideType.BonusTime:
      assertAuthorityContract(
        approval.kind === PolicyApprovalKind.BonusTime,
        'bonus-time overrides require a bonus-time approval request'
      );
      assertAuthorityContract(
        grant.action === PolicyAction.Allow || grant.action === PolicyAction.TimeLimit,
        'bonus-time overrides must keep the action within allow or time-limit'
      );
      assertAuthorityContract(
        grant.bonusTimeMinutes !== null && grant.bonusTimeMinutes > 0,
        'bonus-time overrides must include a positive bonusTimeMinutes value'
      );
      break;
  }

  switch (grant.state) {
    case PolicyOverrideState.Active:
      assertAuthorityContract(evaluatedAt < effectiveUntil, 'active overrides cannot already be past effectiveUntil');
      break;
    case PolicyOverrideState.Expired:
      assertAuthorityContract(
        evaluatedAt >= effectiveUntil,
        'expired overrides require evaluatedAt on or after effectiveUntil'
      );
      break;
    case PolicyOverrideState.Revoked:
      assertAuthorityContract(evaluatedAt >= effectiveFrom, 'revoked overrides require an effectiveFrom boundary');
      break;
  }
}

function assertResolutionHasNoReviewOrOverrideArtifacts(
  resolution: PolicyApprovalResolution,
  message: string
): void {
  assertAuthorityContract(
    resolution.reviewedBy === null &&
      resolution.reviewedAt === null &&
      resolution.auditReferenceId === null &&
      resolution.override === null,
    message
  );
}

function assertResolutionHasNoReviewOverrideOrReplayArtifacts(
  resolution: PolicyApprovalResolution,
  message: string
): void {
  assertResolutionHasNoReviewOrOverrideArtifacts(resolution, message);
  assertAuthorityContract(resolution.replayOfApprovalId === null, message);
}

export function resolvePolicyAuthority(input: PolicyAuthorityRequest): PolicyAuthorityDecision {
  const request = PolicyAuthorityRequestSchema.parse(input);
  const state = request.decision.dryRun
    ? PolicyAuthorityState.DryRun
    : request.source === PolicyAuthoritySource.ParentPolicy
      ? PolicyAuthorityState.Authorized
      : PolicyAuthorityState.EvidenceOnly;

  return PolicyAuthorityDecisionSchema.parse({
    source: request.source,
    state,
    decision: request.decision,
  });
}

export function resolvePolicyApprovalLifecycle(input: unknown): PolicyApprovalResolution {
  const resolution = PolicyApprovalResolutionSchema.parse(input);
  const evaluatedAt = parseTimestampMillis(resolution.evaluatedAt, 'evaluatedAt');

  validatePolicyApprovalRequest(resolution.approval);

  if (resolution.reviewedAt !== null) {
    const reviewedAt = parseTimestampMillis(resolution.reviewedAt, 'reviewedAt');
    assertAuthorityContract(reviewedAt <= evaluatedAt, 'reviewedAt cannot be after evaluatedAt');
  }

  switch (resolution.state) {
    case PolicyApprovalState.Pending:
      assertAuthorityContract(resolution.reviewedBy === null, 'pending approvals cannot have reviewedBy');
      assertAuthorityContract(resolution.reviewedAt === null, 'pending approvals cannot have reviewedAt');
      assertAuthorityContract(resolution.auditReferenceId === null, 'pending approvals cannot have auditReferenceId');
      assertAuthorityContract(resolution.override === null, 'pending approvals cannot create overrides');
      assertAuthorityContract(
        resolution.replayOfApprovalId === null,
        'pending approvals cannot point at replayOfApprovalId'
      );
      break;
    case PolicyApprovalState.PreviewOnly:
      assertAuthorityContract(
        resolution.approval.origin === PolicyApprovalOrigin.AssistantDraft,
        'preview-only approvals require assistant-draft origin'
      );
      assertResolutionHasNoReviewOverrideOrReplayArtifacts(
        resolution,
        'preview-only approvals must remain unconfirmed and override-free'
      );
      break;
    case PolicyApprovalState.ExpiredRequest:
      assertAuthorityContract(
        evaluatedAt >= parseTimestampMillis(resolution.approval.expiresAt, 'approval.expiresAt'),
        'expired-request state requires evaluatedAt on or after approval.expiresAt'
      );
      assertResolutionHasNoReviewOverrideOrReplayArtifacts(
        resolution,
        'expired-request state cannot include review or override artifacts'
      );
      break;
    case PolicyApprovalState.ReplayRejected:
      assertAuthorityContract(
        resolution.replayOfApprovalId !== null,
        'replay-rejected state requires replayOfApprovalId'
      );
      assertResolutionHasNoReviewOrOverrideArtifacts(
        resolution,
        'replay-rejected state cannot include review or override artifacts'
      );
      break;
    case PolicyApprovalState.Denied:
      assertAuthorityContract(resolution.reviewedBy !== null, 'denied approvals require reviewedBy');
      assertAuthorityContract(resolution.reviewedAt !== null, 'denied approvals require reviewedAt');
      assertAuthorityContract(resolution.auditReferenceId !== null, 'denied approvals require auditReferenceId');
      assertAuthorityContract(resolution.override === null, 'denied approvals cannot create overrides');
      assertAuthorityContract(
        resolution.replayOfApprovalId === null,
        'denied approvals cannot point at replayOfApprovalId'
      );
      break;
    case PolicyApprovalState.Approved:
    case PolicyApprovalState.Modified:
      assertAuthorityContract(resolution.reviewedBy !== null, `${resolution.state} approvals require reviewedBy`);
      assertAuthorityContract(resolution.reviewedAt !== null, `${resolution.state} approvals require reviewedAt`);
      assertAuthorityContract(
        resolution.auditReferenceId !== null,
        `${resolution.state} approvals require auditReferenceId`
      );
      assertAuthorityContract(resolution.override !== null, `${resolution.state} approvals require an override grant`);
      assertAuthorityContract(
        resolution.replayOfApprovalId === null,
        `${resolution.state} approvals cannot point at replayOfApprovalId`
      );
      assertAuthorityContract(
        String(resolution.reviewedBy.actorId) !== String(resolution.approval.childProfile.childProfileId),
        'child requests cannot self-approve or self-modify'
      );
      validatePolicyOverrideGrant(resolution.override, resolution.approval, evaluatedAt);
      break;
  }

  return resolution;
}
