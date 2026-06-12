import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentDeviceReferenceSchema, ParentEvidenceReferenceSchema } from '@ocentra-parent/family-domain/references';
import {
  ParentContractSchemaVersionSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import {
  NativeGameBudgetCandidatePolicy,
  NativeGameBudgetDurationSource,
  NativeGameBudgetEvidenceKind,
  NativeGameBudgetRecommendedAction,
  NativeGameBudgetSignalKind,
  NativeGameBudgetSignalPolicyRole,
  nativeGameBudgetAllSessionsMatchPolicyDevice,
  nativeGameBudgetDecisionBudgetMathIsConsistent,
  nativeGameBudgetDecisionCountsAreConsistent,
  nativeGameBudgetRecommendedActionMatchesBudget,
  nativeGameBudgetSignalIsAdvisoryOnly,
} from './native-game-budget-policy-rules';

const NonEmptyNativeGameBudgetText = Schema.String.pipe(Schema.minLength(1));
const NonNegativeNativeGameBudgetNumber = Schema.Number.pipe(
  Schema.filter((value) => (Number.isFinite(value) && value >= 0) || 'Expected a non-negative finite number')
);
const PositiveNativeGameBudgetNumber = Schema.Number.pipe(
  Schema.filter((value) => (Number.isFinite(value) && value > 0) || 'Expected a positive finite number')
);
const OptionalEvidenceReferencesSchema = Schema.Array(ParentEvidenceReferenceSchema);

export const NativeGameBudgetEvidenceKindSchema = withParser(
  Schema.Literal(
    NativeGameBudgetEvidenceKind.KnownGameSession,
    NativeGameBudgetEvidenceKind.LauncherGameCandidate,
    NativeGameBudgetEvidenceKind.LauncherOnly,
    NativeGameBudgetEvidenceKind.PossiblyGame,
    NativeGameBudgetEvidenceKind.UnknownGameLike
  )
);

export const NativeGameBudgetCandidatePolicySchema = withParser(
  Schema.Literal(
    NativeGameBudgetCandidatePolicy.ExcludeCandidates,
    NativeGameBudgetCandidatePolicy.IncludeParentApprovedCandidates,
    NativeGameBudgetCandidatePolicy.ReviewCandidates
  )
);

export const NativeGameBudgetDurationSourceSchema = withParser(
  Schema.Literal(NativeGameBudgetDurationSource.RunningDuration, NativeGameBudgetDurationSource.ForegroundDuration)
);

export const NativeGameBudgetTargetKindSchema = withParser(
  Schema.Literal('all-native-games', 'native-game', 'native-game-category')
);

export const NativeGameBudgetRecommendedActionSchema = withParser(
  Schema.Literal(
    NativeGameBudgetRecommendedAction.Observe,
    NativeGameBudgetRecommendedAction.Warn,
    NativeGameBudgetRecommendedAction.AskParent,
    NativeGameBudgetRecommendedAction.TimeLimitDryRun
  )
);

export const NativeGameBudgetSignalKindSchema = withParser(
  Schema.Literal(
    NativeGameBudgetSignalKind.Rating,
    NativeGameBudgetSignalKind.Ugc,
    NativeGameBudgetSignalKind.Multiplayer,
    NativeGameBudgetSignalKind.Purchase,
    NativeGameBudgetSignalKind.NativeGameCategory
  )
);

export const NativeGameBudgetSignalPolicyRoleSchema = withParser(
  Schema.Literal(
    NativeGameBudgetSignalPolicyRole.ParentPreviewOnly,
    NativeGameBudgetSignalPolicyRole.BudgetTargetingInput,
    NativeGameBudgetSignalPolicyRole.ManualReviewInput,
    NativeGameBudgetSignalPolicyRole.DirectEnforcement
  )
);

export const NativeGameBudgetEnforcementHandoffStateSchema = withParser(
  Schema.Literal('not-requested', 'manual-required')
);

export const NativeGameBudgetPolicyIdSchema = NonEmptyNativeGameBudgetText.pipe(
  Schema.brand('NativeGameBudgetPolicyId')
);
export const NativeGameBudgetSessionRefIdSchema = NonEmptyNativeGameBudgetText.pipe(
  Schema.brand('NativeGameBudgetSessionRefId')
);
export const NativeGameBudgetDecisionIdSchema = NonEmptyNativeGameBudgetText.pipe(
  Schema.brand('NativeGameBudgetDecisionId')
);
export const NativeGameBudgetTargetRefSchema = NonEmptyNativeGameBudgetText.pipe(
  Schema.brand('NativeGameBudgetTargetRef')
);
export const NativeGameBudgetSignalRefSchema = NonEmptyNativeGameBudgetText.pipe(
  Schema.brand('NativeGameBudgetSignalRef')
);

export const NativeGameBudgetTargetSchema = withParser(
  Schema.Struct({
    targetKind: NativeGameBudgetTargetKindSchema,
    targetRef: Schema.Union(NativeGameBudgetTargetRefSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (target) =>
        target.targetKind !== 'all-native-games' ||
        target.targetRef === null ||
        'Expected all-native-games budget targets to avoid a concrete target ref'
    )
  )
);

export const NativeGameBudgetSignalSchema = withParser(
  Schema.Struct({
    signalRef: NativeGameBudgetSignalRefSchema,
    signalKind: NativeGameBudgetSignalKindSchema,
    policyRole: NativeGameBudgetSignalPolicyRoleSchema,
    evidenceReferences: OptionalEvidenceReferencesSchema,
  })
    .pipe(
      Schema.filter(
        (signal) => signal.evidenceReferences.length > 0 || 'Expected native game budget signals to cite evidence'
      )
    )
    .pipe(
      Schema.filter(
        (signal) =>
          nativeGameBudgetSignalIsAdvisoryOnly(signal) ||
          'Expected rating, UGC, multiplayer, and purchase signals to avoid direct enforcement'
      )
    )
);

export const NativeGameBudgetSessionReferenceSchema = withParser(
  Schema.Struct({
    sessionRefId: NativeGameBudgetSessionRefIdSchema,
    device: ParentDeviceReferenceSchema,
    observedAt: ParentTimestampSchema,
  })
);

export const NativeGameBudgetSessionInputSchema = withParser(
  Schema.Struct({
    sessionRef: NativeGameBudgetSessionReferenceSchema,
    evidenceKind: NativeGameBudgetEvidenceKindSchema,
    parentAllowedCandidate: Schema.Boolean,
    runningDurationMs: NonNegativeNativeGameBudgetNumber,
    foregroundDurationMs: NonNegativeNativeGameBudgetNumber,
    evidenceReferences: OptionalEvidenceReferencesSchema,
    advisorySignals: Schema.optionalWith(Schema.Array(NativeGameBudgetSignalSchema), { default: () => [] }),
  }).pipe(
    Schema.filter(
      (session) =>
        session.evidenceReferences.length > 0 || 'Expected native game budget session inputs to cite evidence'
    )
  )
);

export const NativeGameBudgetPolicySchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    budgetPolicyId: NativeGameBudgetPolicyIdSchema,
    policyVersion: ParentPolicyVersionSchema,
    device: ParentDeviceReferenceSchema,
    target: NativeGameBudgetTargetSchema,
    dailyBudgetMinutes: PositiveNativeGameBudgetNumber,
    durationSource: NativeGameBudgetDurationSourceSchema,
    candidatePolicy: NativeGameBudgetCandidatePolicySchema,
    whenExceededAction: NativeGameBudgetRecommendedActionSchema,
    previewEvidenceReferences: OptionalEvidenceReferencesSchema,
  }).pipe(
    Schema.filter(
      (policy) =>
        policy.previewEvidenceReferences.length > 0 || 'Expected native game budget policies to cite preview evidence'
    )
  )
);

export const NativeGameBudgetDryRunDecisionSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    decisionId: NativeGameBudgetDecisionIdSchema,
    policy: NativeGameBudgetPolicySchema,
    sessions: Schema.Array(NativeGameBudgetSessionInputSchema),
    countedSessionRefs: Schema.Array(NativeGameBudgetSessionRefIdSchema),
    excludedSessionRefs: Schema.Array(NativeGameBudgetSessionRefIdSchema),
    countedDurationMs: NonNegativeNativeGameBudgetNumber,
    budgetLimitMs: PositiveNativeGameBudgetNumber,
    budgetExceeded: Schema.Boolean,
    recommendedAction: NativeGameBudgetRecommendedActionSchema,
    dryRun: Schema.Literal(true),
    enforcementHandoffState: NativeGameBudgetEnforcementHandoffStateSchema,
    evidenceReferences: OptionalEvidenceReferencesSchema,
    evaluatedAt: ParentTimestampSchema,
  })
    .pipe(Schema.filter((decision) => decision.sessions.length > 0 || 'Expected budget decisions to include sessions'))
    .pipe(
      Schema.filter(
        (decision) => decision.evidenceReferences.length > 0 || 'Expected native game budget decisions to cite evidence'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          nativeGameBudgetAllSessionsMatchPolicyDevice(decision) ||
          'Expected native game budget decisions to stay on one policy device'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          nativeGameBudgetDecisionCountsAreConsistent(decision) ||
          'Expected counted and excluded native game budget sessions to match launcher policy'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          nativeGameBudgetDecisionBudgetMathIsConsistent(decision) ||
          'Expected native game budget limit and exceeded math to match'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          nativeGameBudgetRecommendedActionMatchesBudget(
            decision.recommendedAction,
            decision.budgetExceeded,
            decision.countedDurationMs
          ) || 'Expected time-limit dry-runs to require counted gameplay over budget'
      )
    )
);

export type NativeGameBudgetEvidenceKind = Infer<typeof NativeGameBudgetEvidenceKindSchema>;
export type NativeGameBudgetCandidatePolicy = Infer<typeof NativeGameBudgetCandidatePolicySchema>;
export type NativeGameBudgetDurationSource = Infer<typeof NativeGameBudgetDurationSourceSchema>;
export type NativeGameBudgetTarget = Infer<typeof NativeGameBudgetTargetSchema>;
export type NativeGameBudgetSignal = Infer<typeof NativeGameBudgetSignalSchema>;
export type NativeGameBudgetSessionInput = Infer<typeof NativeGameBudgetSessionInputSchema>;
export type NativeGameBudgetPolicy = Infer<typeof NativeGameBudgetPolicySchema>;
export type NativeGameBudgetDryRunDecision = Infer<typeof NativeGameBudgetDryRunDecisionSchema>;
