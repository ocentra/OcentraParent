import {
  GeneratedPolicyActionStrictnessRank,
  type GeneratedAppGameCategoryRiskPolicyCandidateAction,
  type GeneratedAppGameCategoryRiskPolicyRouteFamily,
  type GeneratedAppGameCategoryRiskPolicyRouteSourceKind,
  type GeneratedAppGameCategoryRiskPolicyRoutingState,
  type GeneratedPolicyAction,
  type GeneratedPolicyApprovalKind,
  type GeneratedPolicyApprovalOrigin,
  type GeneratedPolicyApprovalState,
  type GeneratedPolicyAuthoritySource,
  type GeneratedPolicyAuthorityState,
  type GeneratedPolicyDecisionHandoffState,
  type GeneratedPolicyOverrideState,
  type GeneratedPolicyOverrideType,
  type GeneratedPolicyPreviewBudgetBoundaryState,
  type GeneratedPolicyPreviewConfirmationState,
  type GeneratedPolicyScheduleBoundaryState,
  type GeneratedPolicyScheduleBudgetCarryoverMode,
  type GeneratedPolicyScheduleBudgetResetKind,
  type GeneratedPolicyScheduleClockSource,
  type GeneratedPolicyScheduleDstResolution,
  type GeneratedPolicyScheduleDstTransition,
  type GeneratedPolicyScheduleOfflineRecoveryState,
} from './generated-policy-control-helpers-contracts';

export type GeneratedPolicyScheduleBudgetResetLike = {
  readonly kind: GeneratedPolicyScheduleBudgetResetKind;
  readonly localTime: string;
  readonly day: string | null;
};

export type GeneratedPolicyScheduleBudgetCarryoverLike = {
  readonly mode: GeneratedPolicyScheduleBudgetCarryoverMode;
  readonly maxMinutes: number | null;
};

export type GeneratedPolicyScheduleTimeBudgetLike = {
  readonly budgetWindowMinutes: number;
  readonly gracePeriodMinutes: number;
  readonly reset: GeneratedPolicyScheduleBudgetResetLike;
  readonly effectiveFrom: string;
  readonly effectiveUntil: string | null;
  readonly carryover: GeneratedPolicyScheduleBudgetCarryoverLike;
};

export type GeneratedPolicyScheduleWindowLike = {
  readonly days: ReadonlyArray<unknown>;
  readonly startLocalTime: string;
  readonly endLocalTime: string;
};

export type GeneratedPolicyScheduleLike = {
  readonly windows: ReadonlyArray<GeneratedPolicyScheduleWindowLike>;
  readonly timeBudget: GeneratedPolicyScheduleTimeBudgetLike;
};

export type GeneratedPolicyScheduleDstBoundaryLike = {
  readonly transition: GeneratedPolicyScheduleDstTransition;
  readonly localTime: string;
  readonly offsetBeforeMinutes: number;
  readonly offsetAfterMinutes: number;
  readonly resolution: GeneratedPolicyScheduleDstResolution;
};

export type GeneratedPolicyScheduleClockSkewLike = {
  readonly observedAt: string;
  readonly allowedSkewMinutes: number;
  readonly observedSkewMinutes: number;
};

export type GeneratedPolicyScheduleExceptionLike = {
  readonly startsAt: string;
  readonly expiresAt: string;
};

export type GeneratedPolicyScheduleExpiryLike = {
  readonly expiresAt: string;
  readonly expiredAt: string;
};

export type GeneratedPolicyScheduleOfflineRecoveryStatusLike = {
  readonly state: GeneratedPolicyScheduleOfflineRecoveryState;
  readonly recoveredAt: string | null;
  readonly recoveredOfflineMinutes: number;
};

export type GeneratedPolicyScheduleTimeBudgetStatusLike = {
  readonly budgetWindowMinutes: number;
  readonly usedMinutes: number;
  readonly remainingMinutes: number;
  readonly carryoverMinutes: number;
  readonly gracePeriodMinutes: number;
  readonly resetAt: string;
  readonly clockSource: GeneratedPolicyScheduleClockSource;
  readonly offlineRecovery: GeneratedPolicyScheduleOfflineRecoveryStatusLike;
  readonly bonusTimeMinutes: number | null;
  readonly bonusTimeRemainingMinutes: number | null;
  readonly bonusTimeExpiresAt: string | null;
};

export type GeneratedPolicyScheduleBoundaryLike = {
  readonly evaluatedAt: string;
  readonly localTime: string;
  readonly state: GeneratedPolicyScheduleBoundaryState;
  readonly dstBoundary: GeneratedPolicyScheduleDstBoundaryLike | null;
  readonly clockSkew: GeneratedPolicyScheduleClockSkewLike | null;
  readonly exception: GeneratedPolicyScheduleExceptionLike | null;
  readonly expiry: GeneratedPolicyScheduleExpiryLike | null;
  readonly timeBudget: GeneratedPolicyScheduleTimeBudgetStatusLike | null;
};

export type GeneratedPolicyDecisionLike = {
  readonly action: GeneratedPolicyAction;
  readonly dryRun: boolean;
  readonly enforcementHandoffState: GeneratedPolicyDecisionHandoffState;
  readonly localAiResultId: string | null;
  readonly evidenceReferences: ReadonlyArray<unknown>;
  readonly ruleIds: ReadonlyArray<string>;
};

export type GeneratedPolicyPreviewLike = {
  readonly confirmationState: GeneratedPolicyPreviewConfirmationState;
  readonly confirmedBy: unknown | null;
  readonly confirmedAt: string | null;
  readonly decision: GeneratedPolicyDecisionLike;
};

export type GeneratedPolicyAuthorityRequestLike<TDecision extends { readonly dryRun: boolean }> = {
  readonly source: GeneratedPolicyAuthoritySource;
  readonly decision: TDecision;
};

export type GeneratedPolicyAuthorityDecisionLike<TDecision> = {
  readonly source: GeneratedPolicyAuthoritySource;
  readonly state: GeneratedPolicyAuthorityState;
  readonly decision: TDecision;
};

export type GeneratedPolicyApprovalRequestLike = {
  readonly origin: GeneratedPolicyApprovalOrigin;
  readonly kind: GeneratedPolicyApprovalKind;
  readonly childProfile: {
    readonly childProfileId: unknown;
  };
  readonly requestedAt: string;
  readonly expiresAt: string;
  readonly requestedBonusTimeMinutes: number | null;
  readonly scheduleBoundary: GeneratedPolicyScheduleBoundaryLike | null;
};

export type GeneratedPolicyOverrideGrantLike = {
  readonly overrideType: GeneratedPolicyOverrideType;
  readonly state: GeneratedPolicyOverrideState;
  readonly action: GeneratedPolicyAction;
  readonly effectiveFrom: string;
  readonly effectiveUntil: string;
  readonly bonusTimeMinutes: number | null;
};

export type GeneratedPolicyApprovalResolutionLike = {
  readonly approval: GeneratedPolicyApprovalRequestLike;
  readonly state: GeneratedPolicyApprovalState;
  readonly evaluatedAt: string;
  readonly reviewedBy: {
    readonly actorId: unknown;
  } | null;
  readonly reviewedAt: string | null;
  readonly auditReferenceId: unknown | null;
  readonly override: GeneratedPolicyOverrideGrantLike | null;
  readonly replayOfApprovalId: unknown | null;
};

export type GeneratedAppGameCategoryRiskPolicyRouteLike = {
  readonly routeFamily: GeneratedAppGameCategoryRiskPolicyRouteFamily;
  readonly sourceKind: GeneratedAppGameCategoryRiskPolicyRouteSourceKind;
  readonly targetKind: string;
  readonly candidateAction: GeneratedAppGameCategoryRiskPolicyCandidateAction;
  readonly requestedAction: string;
  readonly policyAction: GeneratedPolicyAction;
  readonly routingState: GeneratedAppGameCategoryRiskPolicyRoutingState;
  readonly categoryProof: {
    readonly proofKind: string;
    readonly evidenceState: string;
  };
  readonly supportingEvidence: ReadonlyArray<unknown>;
  readonly aiDigestRef: unknown;
};

export type GeneratedScreenAiStricterParentRuleInputLike = {
  readonly sourceDecision: GeneratedPolicyDecisionLike;
  readonly stricterParentRule: {
    readonly enabled: boolean;
    readonly action: GeneratedPolicyAction;
  };
  readonly expectedFinalAction: GeneratedPolicyAction;
};

export type GeneratedScreenAiStricterParentRuleProofLike = {
  readonly finalAction: GeneratedPolicyAction;
  readonly stricterParentRuleAction: GeneratedPolicyAction;
  readonly finalDecision: GeneratedPolicyDecisionLike;
  readonly sourceDecision: GeneratedPolicyDecisionLike;
  readonly stricterParentRule: {
    readonly ruleId: string;
  };
  readonly claimBoundaries: Readonly<Record<string, boolean>>;
};

export function compareGeneratedPolicyActionStrictness(
  left: GeneratedPolicyAction,
  right: GeneratedPolicyAction
): number {
  return GeneratedPolicyActionStrictnessRank[left] - GeneratedPolicyActionStrictnessRank[right];
}

export function selectGeneratedStricterPolicyAction(
  parentRuleAction: GeneratedPolicyAction,
  localAiAction: GeneratedPolicyAction
): GeneratedPolicyAction {
  return compareGeneratedPolicyActionStrictness(parentRuleAction, localAiAction) >= 0
    ? parentRuleAction
    : localAiAction;
}

export function validateGeneratedPolicySchedule(schedule: GeneratedPolicyScheduleLike): void {
  assertGeneratedPolicyContract(schedule.windows.length > 0, 'schedules must define at least one local window');
  for (const [index, window] of schedule.windows.entries()) {
    assertGeneratedPolicyContract(
      window.days.length > 0,
      `schedules must define at least one day for windows[${index}]`
    );
    parseGeneratedLocalTimeMinutes(window.startLocalTime, `windows[${index}].startLocalTime`);
    parseGeneratedLocalTimeMinutes(window.endLocalTime, `windows[${index}].endLocalTime`);
  }
  validateGeneratedPolicyScheduleTimeBudget(schedule.timeBudget);
}

export function validateGeneratedPolicyScheduleBoundary(boundary: GeneratedPolicyScheduleBoundaryLike): void {
  const evaluatedAt = parseGeneratedTimestampMillis(boundary.evaluatedAt, 'evaluatedAt');
  parseGeneratedLocalTimeMinutes(boundary.localTime, 'localTime');
  validateGeneratedPolicyScheduleBoundaryOptionalSections(boundary, evaluatedAt);
  if (boundary.timeBudget !== null) {
    validateGeneratedPolicyScheduleTimeBudgetStatus(boundary.timeBudget, evaluatedAt);
  }

  switch (boundary.state) {
    case 'dst-gap':
      assertGeneratedPolicyContract(boundary.dstBoundary !== null, 'dst-gap boundaries require dstBoundary details');
      assertGeneratedPolicyContract(
        boundary.dstBoundary.transition === 'spring-forward',
        'dst-gap boundaries must use the spring-forward transition'
      );
      assertGeneratedPolicyContract(
        boundary.dstBoundary.resolution !== 'first-occurrence' &&
          boundary.dstBoundary.resolution !== 'second-occurrence',
        'dst-gap boundaries cannot use overlap-only resolutions'
      );
      return;
    case 'dst-overlap':
      assertGeneratedPolicyContract(
        boundary.dstBoundary !== null,
        'dst-overlap boundaries require dstBoundary details'
      );
      assertGeneratedPolicyContract(
        boundary.dstBoundary.transition === 'fall-back',
        'dst-overlap boundaries must use the fall-back transition'
      );
      assertGeneratedPolicyContract(
        boundary.dstBoundary.resolution !== 'skip-forward',
        'dst-overlap boundaries cannot skip the repeated hour'
      );
      return;
    case 'clock-skew':
      assertGeneratedPolicyContract(boundary.clockSkew !== null, 'clock-skew boundaries require clockSkew details');
      assertGeneratedPolicyContract(
        Math.abs(boundary.clockSkew.observedSkewMinutes) > boundary.clockSkew.allowedSkewMinutes,
        'clock-skew boundaries require skew beyond the allowed tolerance'
      );
      return;
    case 'exception-active':
      assertGeneratedPolicyContract(
        boundary.exception !== null,
        'exception-active boundaries require exception details'
      );
      assertGeneratedPolicyContract(
        evaluatedAt >= parseGeneratedTimestampMillis(boundary.exception.startsAt, 'exception.startsAt') &&
          evaluatedAt < parseGeneratedTimestampMillis(boundary.exception.expiresAt, 'exception.expiresAt'),
        'exception-active boundaries must be evaluated inside the exception window'
      );
      return;
    case 'expired':
      assertGeneratedPolicyContract(boundary.expiry !== null, 'expired schedule boundaries require expiry details');
      assertGeneratedPolicyContract(
        evaluatedAt >= parseGeneratedTimestampMillis(boundary.expiry.expiresAt, 'expiry.expiresAt'),
        'expired schedule boundaries must be evaluated on or after expiry'
      );
      return;
    default:
      return;
  }
}

export function validateGeneratedPolicyPreview(preview: GeneratedPolicyPreviewLike): void {
  const hasConfirmedBy = preview.confirmedBy !== null;
  const hasConfirmedAt = preview.confirmedAt !== null;
  assertGeneratedPolicyContract(
    hasConfirmedBy === hasConfirmedAt,
    'preview confirmation requires both confirmedBy and confirmedAt together'
  );
  assertGeneratedPolicyContract(preview.decision.dryRun, 'preview decisions must remain dry-run');
  assertGeneratedPolicyContract(
    preview.decision.enforcementHandoffState === 'disabled',
    'preview decisions must keep enforcement handoff disabled'
  );

  switch (preview.confirmationState) {
    case 'confirmation-required':
      assertGeneratedPolicyContract(
        preview.confirmedBy === null && preview.confirmedAt === null,
        'confirmation-required previews cannot include confirmedBy or confirmedAt'
      );
      return;
    case 'confirmed':
      assertGeneratedPolicyContract(preview.confirmedBy !== null, 'confirmed previews require confirmedBy');
      assertGeneratedPolicyContract(preview.confirmedAt !== null, 'confirmed previews require confirmedAt');
      return;
  }
}

export function resolveGeneratedPolicyAuthority<TDecision extends { readonly dryRun: boolean }>(
  request: GeneratedPolicyAuthorityRequestLike<TDecision>
): GeneratedPolicyAuthorityDecisionLike<TDecision> {
  return {
    source: request.source,
    state: resolveGeneratedPolicyAuthorityState(request.source, request.decision.dryRun),
    decision: request.decision,
  };
}

export function resolveGeneratedPolicyAuthorityState(
  source: GeneratedPolicyAuthoritySource,
  dryRun: boolean
): GeneratedPolicyAuthorityState {
  if (dryRun) {
    return 'dry-run';
  }

  return source === 'parent-policy' ? 'authorized' : 'evidence-only';
}

export function resolveGeneratedPolicyApprovalLifecycle<T extends GeneratedPolicyApprovalResolutionLike>(
  resolution: T
): T {
  const evaluatedAt = parseGeneratedTimestampMillis(resolution.evaluatedAt, 'evaluatedAt');
  validateGeneratedPolicyApprovalRequest(resolution.approval);

  if (resolution.reviewedAt !== null) {
    const reviewedAt = parseGeneratedTimestampMillis(resolution.reviewedAt, 'reviewedAt');
    assertGeneratedPolicyContract(reviewedAt <= evaluatedAt, 'reviewedAt cannot be after evaluatedAt');
  }

  switch (resolution.state) {
    case 'pending':
      assertGeneratedPendingApprovalIsClean(resolution);
      return resolution;
    case 'preview-only':
      assertGeneratedPreviewOnlyApprovalIsHonest(resolution);
      return resolution;
    case 'expired-request':
      assertGeneratedExpiredRequestApprovalIsHonest(resolution, evaluatedAt);
      return resolution;
    case 'replay-rejected':
      assertGeneratedReplayRejectedApprovalIsHonest(resolution);
      return resolution;
    case 'denied':
      assertGeneratedDeniedApprovalIsHonest(resolution);
      return resolution;
    case 'approved':
    case 'modified':
      assertGeneratedResolvedApprovalIsHonest(resolution, evaluatedAt);
      return resolution;
  }
}

function assertGeneratedPendingApprovalIsClean(resolution: GeneratedPolicyApprovalResolutionLike): void {
  assertGeneratedPolicyContract(
    resolution.reviewedBy === null &&
      resolution.reviewedAt === null &&
      resolution.auditReferenceId === null &&
      resolution.override === null &&
      resolution.replayOfApprovalId === null,
    'pending approvals cannot include review, replay, or override artifacts'
  );
}

function assertGeneratedPreviewOnlyApprovalIsHonest(resolution: GeneratedPolicyApprovalResolutionLike): void {
  assertGeneratedPolicyContract(
    resolution.approval.origin === 'assistant-draft',
    'preview-only approvals require assistant-draft origin'
  );
  assertGeneratedResolutionHasNoReviewOverrideOrReplayArtifacts(
    resolution,
    'preview-only approvals must remain unconfirmed and override-free'
  );
}

function assertGeneratedExpiredRequestApprovalIsHonest(
  resolution: GeneratedPolicyApprovalResolutionLike,
  evaluatedAt: number
): void {
  assertGeneratedPolicyContract(
    evaluatedAt >= parseGeneratedTimestampMillis(resolution.approval.expiresAt, 'approval.expiresAt'),
    'expired-request state requires evaluatedAt on or after approval.expiresAt'
  );
  assertGeneratedResolutionHasNoReviewOverrideOrReplayArtifacts(
    resolution,
    'expired-request state cannot include review or override artifacts'
  );
}

function assertGeneratedReplayRejectedApprovalIsHonest(resolution: GeneratedPolicyApprovalResolutionLike): void {
  assertGeneratedPolicyContract(
    resolution.replayOfApprovalId !== null,
    'replay-rejected state requires replayOfApprovalId'
  );
  assertGeneratedResolutionHasNoReviewOrOverrideArtifacts(
    resolution,
    'replay-rejected state cannot include review or override artifacts'
  );
}

function assertGeneratedDeniedApprovalIsHonest(resolution: GeneratedPolicyApprovalResolutionLike): void {
  assertGeneratedPolicyContract(resolution.reviewedBy !== null, 'denied approvals require reviewedBy');
  assertGeneratedPolicyContract(resolution.reviewedAt !== null, 'denied approvals require reviewedAt');
  assertGeneratedPolicyContract(resolution.auditReferenceId !== null, 'denied approvals require auditReferenceId');
  assertGeneratedPolicyContract(resolution.override === null, 'denied approvals cannot create overrides');
  assertGeneratedPolicyContract(
    resolution.replayOfApprovalId === null,
    'denied approvals cannot point at replayOfApprovalId'
  );
}

function assertGeneratedResolvedApprovalIsHonest(
  resolution: GeneratedPolicyApprovalResolutionLike,
  evaluatedAt: number
): void {
  assertGeneratedPolicyContract(
    resolution.reviewedBy !== null &&
      resolution.reviewedAt !== null &&
      resolution.auditReferenceId !== null &&
      resolution.override !== null,
    `${resolution.state} approvals require review, audit, and override artifacts`
  );
  assertGeneratedPolicyContract(
    resolution.replayOfApprovalId === null,
    `${resolution.state} approvals cannot point at replayOfApprovalId`
  );
  assertGeneratedPolicyContract(
    String(resolution.reviewedBy.actorId) !== String(resolution.approval.childProfile.childProfileId),
    'child requests cannot self-approve or self-modify'
  );
  validateGeneratedPolicyOverrideGrant(resolution.override, resolution.approval, evaluatedAt);
}

export function generatedAppGameCategoryRiskPolicyRouteTargetMatchesFamily(
  route: GeneratedAppGameCategoryRiskPolicyRouteLike
): boolean {
  switch (route.routeFamily) {
    case 'nativeApp':
      return route.targetKind === 'app-category';
    case 'riskCandidate':
      return route.targetKind === 'risk-app';
    case 'nativeGame':
      return route.targetKind === 'game-category';
    case 'gameContext':
      return ['multiplayer-game', 'ugc-game', 'purchase-capable-game', 'mature-game'].includes(route.targetKind);
  }
}

export function generatedAppGameCategoryRiskPolicyRouteUsesCategoryProof(
  route: GeneratedAppGameCategoryRiskPolicyRouteLike
): boolean {
  return (
    route.categoryProof.proofKind === 'category-proof' &&
    route.categoryProof.evidenceState === 'active' &&
    route.supportingEvidence.length > 0
  );
}

export function generatedAppGameCategoryRiskPolicyRouteActionMatchesCandidate(
  route: GeneratedAppGameCategoryRiskPolicyRouteLike
): boolean {
  switch (route.candidateAction) {
    case 'observe':
      return route.requestedAction === 'observe' && route.policyAction === 'unknown';
    case 'warn':
      return route.requestedAction === 'warn' && route.policyAction === 'warn';
    case 'askParent':
      return route.requestedAction === 'ask-parent' && route.policyAction === 'ask-parent';
    case 'manualReview':
      return route.requestedAction === 'manual-required' && route.policyAction === 'ask-parent';
  }
}

export function generatedAppGameCategoryRiskPolicyRouteKeepsSoftBoundary(
  route: GeneratedAppGameCategoryRiskPolicyRouteLike
): boolean {
  return ['observe', 'warn', 'ask-parent', 'manual-required'].includes(route.requestedAction);
}

export function generatedAppGameCategoryRiskPolicyRouteManualReviewRequiresManualState(
  route: GeneratedAppGameCategoryRiskPolicyRouteLike
): boolean {
  return route.candidateAction !== 'manualReview' || route.routingState === 'manual-required';
}

export function generatedAppGameCategoryRiskPolicyRouteLocalAiRequiresDigest(
  route: GeneratedAppGameCategoryRiskPolicyRouteLike
): boolean {
  return route.sourceKind !== 'localAi' || route.aiDigestRef !== null;
}

export function generatedScreenAiStricterParentRuleInputIsReady(
  input: GeneratedScreenAiStricterParentRuleInputLike
): boolean {
  return (
    input.sourceDecision.dryRun &&
    input.sourceDecision.enforcementHandoffState !== 'handed-off' &&
    input.sourceDecision.localAiResultId !== null &&
    input.stricterParentRule.enabled &&
    compareGeneratedPolicyActionStrictness(input.stricterParentRule.action, input.sourceDecision.action) > 0 &&
    input.expectedFinalAction ===
      selectGeneratedStricterPolicyAction(input.stricterParentRule.action, input.sourceDecision.action)
  );
}

export function generatedScreenAiStricterParentRuleProofIsHonest(
  proof: GeneratedScreenAiStricterParentRuleProofLike
): boolean {
  return (
    proof.finalAction === proof.stricterParentRuleAction &&
    proof.finalDecision.action === proof.stricterParentRuleAction &&
    proof.finalDecision.localAiResultId === proof.sourceDecision.localAiResultId &&
    proof.finalDecision.evidenceReferences.length === proof.sourceDecision.evidenceReferences.length &&
    proof.finalDecision.ruleIds.includes(proof.stricterParentRule.ruleId) &&
    proof.finalDecision.dryRun &&
    proof.finalDecision.enforcementHandoffState !== 'handed-off' &&
    Object.values(proof.claimBoundaries).every((claim) => claim === false)
  );
}

export function resolveGeneratedPolicyPreviewBudgetBoundaryState(
  boundary: GeneratedPolicyScheduleBoundaryLike | null
): GeneratedPolicyPreviewBudgetBoundaryState {
  if (boundary === null) {
    return 'within-budget';
  }

  if (generatedPolicyPreviewBoundaryNeedsManualResolution(boundary)) {
    return 'manual-required';
  }

  if (boundary.state === 'expired') {
    return 'expired';
  }

  const bonusTimeState = generatedPolicyPreviewBoundaryBonusTimeState(boundary);
  if (bonusTimeState !== null) {
    return bonusTimeState;
  }

  return 'within-budget';
}

function validateGeneratedPolicyScheduleTimeBudget(timeBudget: GeneratedPolicyScheduleTimeBudgetLike): void {
  assertGeneratedPositiveNumber(timeBudget.budgetWindowMinutes, 'timeBudget.budgetWindowMinutes');
  assertGeneratedNonNegativeNumber(timeBudget.gracePeriodMinutes, 'timeBudget.gracePeriodMinutes');
  parseGeneratedLocalTimeMinutes(timeBudget.reset.localTime, 'timeBudget.reset.localTime');

  const effectiveFrom = parseGeneratedTimestampMillis(timeBudget.effectiveFrom, 'timeBudget.effectiveFrom');
  if (timeBudget.effectiveUntil !== null) {
    const effectiveUntil = parseGeneratedTimestampMillis(timeBudget.effectiveUntil, 'timeBudget.effectiveUntil');
    assertGeneratedPolicyContract(
      effectiveUntil > effectiveFrom,
      'timeBudget.effectiveUntil must be after timeBudget.effectiveFrom'
    );
  }

  if (timeBudget.reset.kind === 'weekly') {
    assertGeneratedPolicyContract(timeBudget.reset.day !== null, 'weekly reset rules require timeBudget.reset.day');
  } else {
    assertGeneratedPolicyContract(
      timeBudget.reset.day === null,
      'non-weekly reset rules cannot set timeBudget.reset.day'
    );
  }

  switch (timeBudget.carryover.mode) {
    case 'discard-unused':
      assertGeneratedPolicyContract(
        timeBudget.carryover.maxMinutes === null,
        'discard-unused carryover cannot set timeBudget.carryover.maxMinutes'
      );
      return;
    case 'carry-forward':
      if (timeBudget.carryover.maxMinutes !== null) {
        assertGeneratedPositiveNumber(timeBudget.carryover.maxMinutes, 'timeBudget.carryover.maxMinutes');
      }
      return;
    case 'cap-carryover':
      assertGeneratedPolicyContract(
        timeBudget.carryover.maxMinutes !== null,
        'cap-carryover requires timeBudget.carryover.maxMinutes'
      );
      assertGeneratedPositiveNumber(timeBudget.carryover.maxMinutes ?? 0, 'timeBudget.carryover.maxMinutes');
      return;
  }
}

function validateGeneratedPolicyScheduleTimeBudgetStatus(
  timeBudget: GeneratedPolicyScheduleTimeBudgetStatusLike,
  evaluatedAt: number
): void {
  assertGeneratedPositiveNumber(timeBudget.budgetWindowMinutes, 'timeBudget.budgetWindowMinutes');
  assertGeneratedNonNegativeNumber(timeBudget.usedMinutes, 'timeBudget.usedMinutes');
  assertGeneratedNonNegativeNumber(timeBudget.remainingMinutes, 'timeBudget.remainingMinutes');
  assertGeneratedNonNegativeNumber(timeBudget.carryoverMinutes, 'timeBudget.carryoverMinutes');
  assertGeneratedNonNegativeNumber(timeBudget.gracePeriodMinutes, 'timeBudget.gracePeriodMinutes');

  const resetAt = parseGeneratedTimestampMillis(timeBudget.resetAt, 'timeBudget.resetAt');
  assertGeneratedPolicyContract(resetAt > evaluatedAt, 'timeBudget.resetAt must be after evaluatedAt');

  switch (timeBudget.offlineRecovery.state) {
    case 'not-needed':
      assertGeneratedPolicyContract(
        timeBudget.offlineRecovery.recoveredAt === null,
        'offline recovery state not-needed cannot include recoveredAt'
      );
      assertGeneratedPolicyContract(
        timeBudget.offlineRecovery.recoveredOfflineMinutes === 0,
        'offline recovery state not-needed cannot recover offline minutes'
      );
      break;
    case 'recovered-from-device':
    case 'recomputed-from-journal':
      assertGeneratedPolicyContract(
        timeBudget.offlineRecovery.recoveredAt !== null,
        'recovered offline timer states require recoveredAt'
      );
      parseGeneratedTimestampMillis(timeBudget.offlineRecovery.recoveredAt ?? '', 'offlineRecovery.recoveredAt');
      break;
    case 'manual-required':
      if (timeBudget.offlineRecovery.recoveredAt !== null) {
        parseGeneratedTimestampMillis(timeBudget.offlineRecovery.recoveredAt, 'offlineRecovery.recoveredAt');
      }
      break;
  }

  assertGeneratedNonNegativeNumber(
    timeBudget.offlineRecovery.recoveredOfflineMinutes,
    'offlineRecovery.recoveredOfflineMinutes'
  );

  if (timeBudget.bonusTimeMinutes === null) {
    assertGeneratedPolicyContract(
      timeBudget.bonusTimeRemainingMinutes === null,
      'timeBudget.bonusTimeRemainingMinutes requires bonusTimeMinutes'
    );
    assertGeneratedPolicyContract(
      timeBudget.bonusTimeExpiresAt === null,
      'timeBudget.bonusTimeExpiresAt requires bonusTimeMinutes'
    );
    return;
  }

  assertGeneratedPositiveNumber(timeBudget.bonusTimeMinutes, 'timeBudget.bonusTimeMinutes');
  assertGeneratedPolicyContract(
    timeBudget.bonusTimeRemainingMinutes !== null,
    'timeBudget.bonusTimeRemainingMinutes is required when bonusTimeMinutes are active'
  );
  assertGeneratedNonNegativeNumber(timeBudget.bonusTimeRemainingMinutes ?? 0, 'timeBudget.bonusTimeRemainingMinutes');
  assertGeneratedPolicyContract(
    (timeBudget.bonusTimeRemainingMinutes ?? 0) <= timeBudget.bonusTimeMinutes,
    'timeBudget.bonusTimeRemainingMinutes cannot exceed timeBudget.bonusTimeMinutes'
  );
  assertGeneratedPolicyContract(
    timeBudget.bonusTimeExpiresAt !== null,
    'timeBudget.bonusTimeExpiresAt is required when bonusTimeMinutes are active'
  );
  assertGeneratedPolicyContract(
    parseGeneratedTimestampMillis(timeBudget.bonusTimeExpiresAt ?? '', 'timeBudget.bonusTimeExpiresAt') > evaluatedAt,
    'timeBudget.bonusTimeExpiresAt must be after evaluatedAt while bonus time is active'
  );
}

function validateGeneratedPolicyScheduleBoundaryOptionalSections(
  boundary: GeneratedPolicyScheduleBoundaryLike,
  evaluatedAt: number
): void {
  if (boundary.dstBoundary !== null) {
    parseGeneratedLocalTimeMinutes(boundary.dstBoundary.localTime, 'dstBoundary.localTime');
    assertGeneratedPolicyContract(
      Number.isInteger(boundary.dstBoundary.offsetBeforeMinutes),
      'dstBoundary.offsetBeforeMinutes must be an integer minute offset'
    );
    assertGeneratedPolicyContract(
      Number.isInteger(boundary.dstBoundary.offsetAfterMinutes),
      'dstBoundary.offsetAfterMinutes must be an integer minute offset'
    );
  }

  if (boundary.clockSkew !== null) {
    parseGeneratedTimestampMillis(boundary.clockSkew.observedAt, 'clockSkew.observedAt');
    assertGeneratedNonNegativeNumber(boundary.clockSkew.allowedSkewMinutes, 'clockSkew.allowedSkewMinutes');
    assertGeneratedPolicyContract(
      Number.isFinite(boundary.clockSkew.observedSkewMinutes),
      'clockSkew.observedSkewMinutes must be a finite number'
    );
  }

  if (boundary.exception !== null) {
    const startsAt = parseGeneratedTimestampMillis(boundary.exception.startsAt, 'exception.startsAt');
    const expiresAt = parseGeneratedTimestampMillis(boundary.exception.expiresAt, 'exception.expiresAt');
    assertGeneratedPolicyContract(expiresAt > startsAt, 'schedule exceptions must expire after they start');
  }

  if (boundary.expiry !== null) {
    const expiresAt = parseGeneratedTimestampMillis(boundary.expiry.expiresAt, 'expiry.expiresAt');
    const expiredAt = parseGeneratedTimestampMillis(boundary.expiry.expiredAt, 'expiry.expiredAt');
    assertGeneratedPolicyContract(expiredAt >= expiresAt, 'expiry.expiredAt must be on or after expiry.expiresAt');
    if (boundary.state !== 'expired') {
      assertGeneratedPolicyContract(
        evaluatedAt < expiresAt,
        'non-expired schedule boundaries cannot be evaluated after expiry'
      );
    }
  }
}

function validateGeneratedPolicyApprovalRequest(request: GeneratedPolicyApprovalRequestLike): void {
  const requestedAt = parseGeneratedTimestampMillis(request.requestedAt, 'approval.requestedAt');
  const expiresAt = parseGeneratedTimestampMillis(request.expiresAt, 'approval.expiresAt');
  assertGeneratedPolicyContract(expiresAt > requestedAt, 'approval.expiresAt must be after approval.requestedAt');

  if (request.scheduleBoundary !== null) {
    validateGeneratedPolicyScheduleBoundary(request.scheduleBoundary);
  }

  if (request.kind === 'bonus-time') {
    assertGeneratedPolicyContract(
      request.requestedBonusTimeMinutes !== null && request.requestedBonusTimeMinutes > 0,
      'bonus-time requests must include a positive requestedBonusTimeMinutes value'
    );
    assertGeneratedPolicyContract(
      request.scheduleBoundary !== null,
      'bonus-time requests must include scheduleBoundary details'
    );
    assertGeneratedPolicyContract(
      request.scheduleBoundary?.timeBudget !== null,
      'bonus-time requests must include scheduleBoundary.timeBudget details'
    );
    return;
  }

  assertGeneratedPolicyContract(
    request.requestedBonusTimeMinutes === null,
    'only bonus-time requests may include requestedBonusTimeMinutes'
  );
}

function validateGeneratedPolicyOverrideGrant(
  grant: GeneratedPolicyOverrideGrantLike,
  approval: GeneratedPolicyApprovalRequestLike,
  evaluatedAt: number
): void {
  const effectiveFrom = parseGeneratedTimestampMillis(grant.effectiveFrom, 'override.effectiveFrom');
  const effectiveUntil = parseGeneratedTimestampMillis(grant.effectiveUntil, 'override.effectiveUntil');
  assertGeneratedPolicyContract(
    effectiveUntil > effectiveFrom,
    'override.effectiveUntil must be after override.effectiveFrom'
  );

  switch (grant.overrideType) {
    case 'temporary-allow':
      assertGeneratedPolicyContract(grant.action === 'allow', 'temporary-allow overrides must resolve to allow');
      assertGeneratedPolicyContract(
        grant.bonusTimeMinutes === null,
        'temporary-allow overrides cannot carry bonusTimeMinutes'
      );
      break;
    case 'temporary-block':
      assertGeneratedPolicyContract(grant.action === 'block', 'temporary-block overrides must resolve to block');
      assertGeneratedPolicyContract(
        grant.bonusTimeMinutes === null,
        'temporary-block overrides cannot carry bonusTimeMinutes'
      );
      break;
    case 'bonus-time':
      assertGeneratedPolicyContract(
        approval.kind === 'bonus-time',
        'bonus-time overrides require a bonus-time approval request'
      );
      assertGeneratedPolicyContract(
        grant.action === 'allow' || grant.action === 'time-limit',
        'bonus-time overrides must keep the action within allow or time-limit'
      );
      assertGeneratedPolicyContract(
        grant.bonusTimeMinutes !== null && grant.bonusTimeMinutes > 0,
        'bonus-time overrides must include a positive bonusTimeMinutes value'
      );
      break;
  }

  switch (grant.state) {
    case 'active':
      assertGeneratedPolicyContract(
        evaluatedAt < effectiveUntil,
        'active overrides cannot already be past effectiveUntil'
      );
      return;
    case 'expired':
      assertGeneratedPolicyContract(
        evaluatedAt >= effectiveUntil,
        'expired overrides require evaluatedAt on or after effectiveUntil'
      );
      return;
    case 'revoked':
      assertGeneratedPolicyContract(
        evaluatedAt >= effectiveFrom,
        'revoked overrides require an effectiveFrom boundary'
      );
      return;
  }
}

function assertGeneratedResolutionHasNoReviewOrOverrideArtifacts(
  resolution: GeneratedPolicyApprovalResolutionLike,
  message: string
): void {
  assertGeneratedPolicyContract(
    resolution.reviewedBy === null &&
      resolution.reviewedAt === null &&
      resolution.auditReferenceId === null &&
      resolution.override === null,
    message
  );
}

function assertGeneratedResolutionHasNoReviewOverrideOrReplayArtifacts(
  resolution: GeneratedPolicyApprovalResolutionLike,
  message: string
): void {
  assertGeneratedResolutionHasNoReviewOrOverrideArtifacts(resolution, message);
  assertGeneratedPolicyContract(resolution.replayOfApprovalId === null, message);
}

function generatedPolicyPreviewBoundaryNeedsManualResolution(boundary: GeneratedPolicyScheduleBoundaryLike): boolean {
  return [
    boundary.state === 'clock-skew',
    (boundary.state === 'dst-gap' || boundary.state === 'dst-overlap') &&
      boundary.dstBoundary?.resolution === 'manual-required',
    boundary.timeBudget?.clockSource === 'manual-required',
    boundary.timeBudget?.offlineRecovery.state === 'manual-required',
  ].some(Boolean);
}

function generatedPolicyPreviewBoundaryBonusTimeState(
  boundary: GeneratedPolicyScheduleBoundaryLike
): GeneratedPolicyPreviewBudgetBoundaryState | null {
  if (boundary.timeBudget?.bonusTimeMinutes === null || boundary.timeBudget?.bonusTimeMinutes === undefined) {
    return null;
  }

  const bonusTimeRemainingMinutes =
    boundary.timeBudget.bonusTimeRemainingMinutes ?? boundary.timeBudget.bonusTimeMinutes;
  return bonusTimeRemainingMinutes < boundary.timeBudget.bonusTimeMinutes ? 'bonus-time-expiring' : 'bonus-time-active';
}

function assertGeneratedPolicyContract(condition: unknown, message: string): asserts condition {
  if (!condition) {
    throw new Error(message);
  }
}

function parseGeneratedTimestampMillis(timestamp: string, fieldName: string): number {
  const millis = Date.parse(timestamp);
  assertGeneratedPolicyContract(!Number.isNaN(millis), `${fieldName} must be an ISO-8601 timestamp`);
  return millis;
}

function parseGeneratedLocalTimeMinutes(localTime: string, fieldName: string): number {
  const match = /^(?<hour>[01]\d|2[0-3]):(?<minute>[0-5]\d)$/.exec(localTime);
  assertGeneratedPolicyContract(match !== null, `${fieldName} must use HH:MM 24-hour local time`);
  const hour = Number(match.groups?.['hour'] ?? '0');
  const minute = Number(match.groups?.['minute'] ?? '0');
  return hour * 60 + minute;
}

function assertGeneratedNonNegativeNumber(value: number, fieldName: string): void {
  assertGeneratedPolicyContract(Number.isFinite(value) && value >= 0, `${fieldName} must be a non-negative number`);
}

function assertGeneratedPositiveNumber(value: number, fieldName: string): void {
  assertGeneratedPolicyContract(Number.isFinite(value) && value > 0, `${fieldName} must be a positive number`);
}
