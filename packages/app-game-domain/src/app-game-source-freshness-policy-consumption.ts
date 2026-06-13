import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  appGameSourceFreshnessReadinessIsPolicyReady,
  appGameSourceFreshnessRequirementFailure,
  appGameSourceFreshnessRowsForRequirement,
  appGameSourceFreshnessTargetAllowsNullRef,
} from './app-game-source-freshness-policy-consumption-rules';
import {
  AppGameSourceFreshnessAdapterDispatchState,
  AppGameSourceFreshnessCapabilityStatus,
  AppGameSourceFreshnessPolicyConsumptionMatrixId,
  AppGameSourceFreshnessPolicyReadinessState,
  AppGameSourceFreshnessPolicyTargetKind,
  AppGameSourceFreshnessReadModelState,
  AppGameSourceFreshnessReasonCode,
  AppGameSourceFreshnessRequirementKind,
  AppGameSourceFreshnessRequirementState,
  AppGameSourceFreshnessSourceKind,
} from './app-game-source-freshness-policy-consumption-values';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
const NonNegativeSourceFreshnessNumber = Schema.Number.pipe(
  Schema.filter((value) => (Number.isFinite(value) && value >= 0) || 'Expected a non-negative finite number')
);
const PositiveSourceFreshnessNumber = Schema.Number.pipe(
  Schema.filter((value) => (Number.isFinite(value) && value > 0) || 'Expected a positive finite number')
);

export const AppGameSourceFreshnessPolicyRequestIdSchema = brandedNonEmptyStringSchema('AppGameSourceFreshnessPolicyRequestId');
export const AppGameSourceFreshnessPolicyReadinessIdSchema = brandedNonEmptyStringSchema('AppGameSourceFreshnessPolicyReadinessId');
export const AppGameSourceFreshnessTargetRefSchema = brandedNonEmptyStringSchema('AppGameSourceFreshnessTargetRef');
export const AppGameSourceFreshnessEvidenceRefSchema = brandedNonEmptyStringSchema('AppGameSourceFreshnessEvidenceRef');
export const AppGameSourceFreshnessMatrixIdSchema = brandedNonEmptyStringSchema('AppGameSourceFreshnessMatrixId');

export const AppGameSourceFreshnessPolicyTargetKindSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceFreshnessPolicyTargetKind))
);
export const AppGameSourceFreshnessRequirementKindSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceFreshnessRequirementKind))
);
export const AppGameSourceFreshnessSourceKindSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceFreshnessSourceKind))
);
export const AppGameSourceFreshnessReadModelStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceFreshnessReadModelState))
);
export const AppGameSourceFreshnessCapabilityStatusSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceFreshnessCapabilityStatus))
);
export const AppGameSourceFreshnessRequirementStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceFreshnessRequirementState))
);
export const AppGameSourceFreshnessPolicyReadinessStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceFreshnessPolicyReadinessState))
);
export const AppGameSourceFreshnessAdapterDispatchStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceFreshnessAdapterDispatchState))
);
export const AppGameSourceFreshnessReasonCodeSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceFreshnessReasonCode))
);

export const AppGameSourceFreshnessStatusRowSchema = withParser(
  Schema.Struct({
    sourceKind: AppGameSourceFreshnessSourceKindSchema,
    state: AppGameSourceFreshnessReadModelStateSchema,
    rowCount: NonNegativeSourceFreshnessNumber,
    lastObservedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    capabilityStatus: AppGameSourceFreshnessCapabilityStatusSchema,
    evidence: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  }).pipe(
    Schema.filter(
      (row) => row.rowCount === 0 || row.evidence.length > 0 || 'Expected non-empty source rows to cite evidence refs'
    )
  )
);

export const AppGameSourceFreshnessPolicyTargetSchema = withParser(
  Schema.Struct({
    targetKind: AppGameSourceFreshnessPolicyTargetKindSchema,
    targetRef: Schema.Union(AppGameSourceFreshnessTargetRefSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (target) =>
        (appGameSourceFreshnessTargetAllowsNullRef(target) && target.targetRef === null) ||
        (!appGameSourceFreshnessTargetAllowsNullRef(target) && target.targetRef !== null) ||
        'Expected aggregate source freshness targets to omit refs and concrete app/game targets to include refs'
    )
  )
);

export const AppGameSourceFreshnessPolicyRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    policyRequestId: AppGameSourceFreshnessPolicyRequestIdSchema,
    target: AppGameSourceFreshnessPolicyTargetSchema,
    requiredSources: Schema.Array(AppGameSourceFreshnessRequirementKindSchema),
    maxSourceAgeMs: PositiveSourceFreshnessNumber,
    sourceStatusRows: Schema.Array(AppGameSourceFreshnessStatusRowSchema),
    requestedAt: ParentTimestampSchema,
    sourceRowsFromActivityReadModel: Schema.Literal(true),
    rawPrivateSourceRowsIncluded: Schema.Literal(false),
  })
    .pipe(
      Schema.filter(
        (request) =>
          request.requiredSources.length > 0 || 'Expected app/game policy readiness to name source requirements'
      )
    )
    .pipe(
      Schema.filter(
        (request) =>
          new Set(request.requiredSources).size === request.requiredSources.length ||
          'Expected app/game source freshness requirements to be unique'
      )
    )
);

export const AppGameSourceFreshnessRequirementResultSchema = withParser(
  Schema.Struct({
    requirementKind: AppGameSourceFreshnessRequirementKindSchema,
    requirementState: AppGameSourceFreshnessRequirementStateSchema,
    reasonCode: Schema.Union(AppGameSourceFreshnessReasonCodeSchema, Schema.Null),
    matchedSourceKinds: Schema.Array(AppGameSourceFreshnessSourceKindSchema),
    sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
    lastObservedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (result) =>
        result.requirementState !== AppGameSourceFreshnessRequirementState.Satisfied ||
        (result.reasonCode === null && result.sourceEvidenceRefs.length > 0) ||
        'Expected satisfied source freshness requirements to cite evidence and carry no failure reason'
    )
  )
);

export const AppGameSourceFreshnessPolicyReadinessSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readinessId: AppGameSourceFreshnessPolicyReadinessIdSchema,
    request: AppGameSourceFreshnessPolicyRequestSchema,
    readinessState: AppGameSourceFreshnessPolicyReadinessStateSchema,
    requirementResults: Schema.Array(AppGameSourceFreshnessRequirementResultSchema),
    policyEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
    policyCompileAllowed: Schema.Boolean,
    adapterDispatchState: AppGameSourceFreshnessAdapterDispatchStateSchema,
    directAdapterCallRequested: Schema.Literal(false),
    rawPrivateSourceRowsIncluded: Schema.Literal(false),
    evaluatedAt: ParentTimestampSchema,
  })
    .pipe(
      Schema.filter(
        (readiness) =>
          readiness.requirementResults.length === readiness.request.requiredSources.length ||
          'Expected one source freshness result per required source'
      )
    )
    .pipe(
      Schema.filter(
        (readiness) =>
          readiness.readinessState !== AppGameSourceFreshnessPolicyReadinessState.PolicyReady ||
          appGameSourceFreshnessReadinessIsPolicyReady(readiness) ||
          'Expected policy-ready source freshness to satisfy all requirements without adapter dispatch'
      )
    )
    .pipe(
      Schema.filter(
        (readiness) =>
          readiness.readinessState !== AppGameSourceFreshnessPolicyReadinessState.ManualRequired ||
          (!readiness.policyCompileAllowed &&
            readiness.requirementResults.some(
              (result) => result.requirementState !== AppGameSourceFreshnessRequirementState.Satisfied
            )) ||
          'Expected manual-required source freshness to block policy compile and name failing source rows'
      )
    )
);

export const AppGameSourceFreshnessPolicyConsumptionMatrixSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    matrixId: AppGameSourceFreshnessMatrixIdSchema,
    generatedAt: ParentTimestampSchema,
    readiness: Schema.Array(AppGameSourceFreshnessPolicyReadinessSchema),
  }).pipe(
    Schema.filter(
      (matrix) => matrix.matrixId === AppGameSourceFreshnessPolicyConsumptionMatrixId && matrix.readiness.length > 0
    )
  )
);

export type AppGameSourceFreshnessStatusRow = Infer<typeof AppGameSourceFreshnessStatusRowSchema>;
export type AppGameSourceFreshnessPolicyTarget = Infer<typeof AppGameSourceFreshnessPolicyTargetSchema>;
export type AppGameSourceFreshnessPolicyRequest = Infer<typeof AppGameSourceFreshnessPolicyRequestSchema>;
export type AppGameSourceFreshnessRequirementResult = Infer<typeof AppGameSourceFreshnessRequirementResultSchema>;
export type AppGameSourceFreshnessPolicyReadiness = Infer<typeof AppGameSourceFreshnessPolicyReadinessSchema>;
export type AppGameSourceFreshnessPolicyConsumptionMatrix = Infer<
  typeof AppGameSourceFreshnessPolicyConsumptionMatrixSchema
>;

const decodeAppGameSourceFreshnessPolicyReadinessId = Schema.decodeUnknownSync(
  AppGameSourceFreshnessPolicyReadinessIdSchema
);
const decodeParentTimestamp = Schema.decodeUnknownSync(ParentTimestampSchema);

export function evaluateAppGameSourceFreshnessPolicyReadiness(
  requestInput: unknown,
  readinessIdInput: unknown,
  evaluatedAtInput: unknown
): AppGameSourceFreshnessPolicyReadiness {
  const request = AppGameSourceFreshnessPolicyRequestSchema.parse(requestInput);
  const readinessId = decodeAppGameSourceFreshnessPolicyReadinessId(readinessIdInput);
  const evaluatedAt = decodeParentTimestamp(evaluatedAtInput);
  const requirementResults = request.requiredSources.map((requirementKind) =>
    evaluateRequirement(request, requirementKind, evaluatedAt)
  );
  const policyEvidenceRefs = [...new Set(requirementResults.flatMap((result) => result.sourceEvidenceRefs))];
  const allSatisfied = requirementResults.every(
    (result) => result.requirementState === AppGameSourceFreshnessRequirementState.Satisfied
  );

  return AppGameSourceFreshnessPolicyReadinessSchema.parse({
    schemaVersion: request.schemaVersion,
    readinessId,
    request,
    readinessState: allSatisfied
      ? AppGameSourceFreshnessPolicyReadinessState.PolicyReady
      : AppGameSourceFreshnessPolicyReadinessState.ManualRequired,
    requirementResults,
    policyEvidenceRefs,
    policyCompileAllowed: allSatisfied,
    adapterDispatchState: AppGameSourceFreshnessAdapterDispatchState.NotDispatched,
    directAdapterCallRequested: false,
    rawPrivateSourceRowsIncluded: false,
    evaluatedAt,
  });
}

function evaluateRequirement(
  request: AppGameSourceFreshnessPolicyRequest,
  requirementKind: typeof AppGameSourceFreshnessRequirementKindSchema.Type,
  evaluatedAt: typeof ParentTimestampSchema.Type
): AppGameSourceFreshnessRequirementResult {
  const rows = appGameSourceFreshnessRowsForRequirement(request.sourceStatusRows, requirementKind);

  if (rows.length === 0) {
    return AppGameSourceFreshnessRequirementResultSchema.parse({
      requirementKind,
      requirementState: AppGameSourceFreshnessRequirementState.Missing,
      reasonCode: AppGameSourceFreshnessReasonCode.MissingSourceStatusRow,
      matchedSourceKinds: [],
      sourceEvidenceRefs: [],
      lastObservedAt: null,
    });
  }

  const freshRow = rows.find(
    (row) =>
      row.state === AppGameSourceFreshnessReadModelState.Ready &&
      row.capabilityStatus === AppGameSourceFreshnessCapabilityStatus.Available &&
      row.rowCount > 0 &&
      row.evidence.length > 0 &&
      appGameSourceFreshnessRequirementFailure(row, evaluatedAt, request.maxSourceAgeMs) === null
  );

  if (freshRow !== undefined) {
    return AppGameSourceFreshnessRequirementResultSchema.parse({
      requirementKind,
      requirementState: AppGameSourceFreshnessRequirementState.Satisfied,
      reasonCode: null,
      matchedSourceKinds: [freshRow.sourceKind],
      sourceEvidenceRefs: freshRow.evidence,
      lastObservedAt: freshRow.lastObservedAt,
    });
  }

  const firstRow = rows[0];

  if (firstRow === undefined) {
    throw new Error('Expected app/game source freshness rows after empty-row guard');
  }

  const firstFailure = appGameSourceFreshnessRequirementFailure(firstRow, evaluatedAt, request.maxSourceAgeMs) ?? {
    requirementState: AppGameSourceFreshnessRequirementState.MissingEvidence,
    reasonCode: AppGameSourceFreshnessReasonCode.MissingSourceEvidence,
  };

  return AppGameSourceFreshnessRequirementResultSchema.parse({
    requirementKind,
    requirementState: firstFailure.requirementState,
    reasonCode: firstFailure.reasonCode,
    matchedSourceKinds: rows.map((row) => row.sourceKind),
    sourceEvidenceRefs: rows.flatMap((row) => row.evidence),
    lastObservedAt: firstRow.lastObservedAt,
  });
}

export const decodeAppGameSourceFreshnessPolicyReadiness = Schema.decodeUnknownSync(
  AppGameSourceFreshnessPolicyReadinessSchema
);
export const decodeAppGameSourceFreshnessPolicyConsumptionMatrix = Schema.decodeUnknownSync(
  AppGameSourceFreshnessPolicyConsumptionMatrixSchema
);

