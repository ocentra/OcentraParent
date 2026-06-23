import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerProofRefSchema,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffRowSchema as ResponseConsumerHandoffRowSchema,
} from './app-game-timer-service-read-api-response-consumer-handoff';
import {
  AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffNoClaimFlags,
  AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState,
  RequiredAppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffNonClaims,
  appGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffCountsMatch,
  appGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffHasNoRuntimeClaims,
  appGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffMatchesResponseConsumerHandoff,
  type AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffStateValue,
} from './app-game-timer-service-read-api-response-consumer-parent-surface-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';

type ResponseConsumerHandoffRow = Infer<typeof ResponseConsumerHandoffRowSchema>;

export const AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffIdSchema = brandedNonEmptyStringSchema(
  'AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffId'
);
export const AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffRowIdSchema = brandedNonEmptyStringSchema(
  'AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffRowId'
);
export const AppGameTimerServiceReadApiResponseConsumerParentSurfaceProofRefSchema = brandedNonEmptyStringSchema(
  'AppGameTimerServiceReadApiResponseConsumerParentSurfaceProofRef'
);
export const AppGameTimerServiceReadApiResponseConsumerParentSurfaceContractRefSchema = brandedNonEmptyStringSchema(
  'AppGameTimerServiceReadApiResponseConsumerParentSurfaceContractRef'
);

const ParentSurfaceHandoffStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState))
);
const ParentSurfaceHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffNonClaims)
);

export const AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    parentSurfaceHandoffId: AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(AppGameTimerServiceReadApiResponseConsumerParentSurfaceContractRefSchema),
    parentSurfaceProofRefs: Schema.Array(AppGameTimerServiceReadApiResponseConsumerParentSurfaceProofRefSchema),
  }).pipe(
    Schema.filter(
      (options) =>
        (options.sourceContractRefs.length > 0 && options.parentSurfaceProofRefs.length > 0) ||
        'Expected parent-surface handoff options to cite source contracts and future parent-surface proof refs'
    )
  )
);

const ParentSurfaceHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffRowIdSchema,
  sourceResponseConsumerHandoffRowId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  parentSurfaceHandoffState: ParentSurfaceHandoffStateSchema,
  inheritedServiceReadApiResponseConsumerProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerProofRefSchema
  ),
  requiredParentSurfaceProofRefs: Schema.Array(AppGameTimerServiceReadApiResponseConsumerParentSurfaceProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  ...Object.fromEntries(
    Object.keys(AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffNoClaimFlags).map((key) => [
      key,
      Schema.Literal(false),
    ])
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffRowSchema = withParser(
  ParentSurfaceHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.parentSurfaceHandoffState !==
          AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired ||
        row.requiredParentSurfaceProofRefs.length > 0 ||
        'Expected parent-surface proof rows to cite required parent-surface proof refs'
    )
  )
);

const ParentSurfaceHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  parentSurfaceHandoffId: AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffIdSchema,
  sourceResponseConsumerHandoffId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(AppGameTimerServiceReadApiResponseConsumerParentSurfaceContractRefSchema),
  rows: Schema.Array(AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  parentSurfaceProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  parentSurfaceHandoffNonClaims: Schema.Array(ParentSurfaceHandoffNonClaimSchema),
  ...Object.fromEntries(
    Object.keys(AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffNoClaimFlags).map((key) => [
      key,
      Schema.Literal(false),
    ])
  ),
});

export const AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffSchema = withParser(
  ParentSurfaceHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        appGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffCountsMatch(handoff) ||
        'Expected parent-surface handoff counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (handoff) =>
        appGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffHasNoRuntimeClaims(handoff) ||
        'Expected parent-surface handoff to keep runtime and rendering claims false'
    )
  )
);

export function buildAppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoff(
  options: Infer<typeof AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffOptionsSchema>,
  responseConsumerHandoff: Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffSchema
  >
): Infer<typeof AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffSchema> {
  const parsedOptions = AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffOptionsSchema.parse(options);
  const parsedResponseConsumerHandoff =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffSchema.parse(
      responseConsumerHandoff
    );
  const rows = parsedResponseConsumerHandoff.rows.map((row, index) =>
    buildParentSurfaceHandoffRow(parsedOptions, row, index)
  );
  return AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffSchema.parse({
    schemaVersion: parsedOptions.schemaVersion,
    parentSurfaceHandoffId: parsedOptions.parentSurfaceHandoffId,
    sourceResponseConsumerHandoffId:
      parsedResponseConsumerHandoff.parentSurfaceReadModelServiceReadApiResponseConsumerHandoffId,
    generatedAt: parsedOptions.generatedAt,
    sourceContractRefs: parsedOptions.sourceContractRefs,
    rows,
    nativeAppRowCount: rows.filter((row) => row.targetDomain === 'native-app').length,
    nativeGameRowCount: rows.filter((row) => row.targetDomain === 'native-game').length,
    parentSurfaceProofRequiredCount: rows.filter(
      (row) =>
        row.parentSurfaceHandoffState ===
        AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.parentSurfaceHandoffState ===
        AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.parentSurfaceHandoffState ===
        AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.BlockedByCompilerDecision
    ).length,
    parentSurfaceHandoffNonClaims: RequiredAppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffNonClaims,
    ...AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffNoClaimFlags,
  });
}

function buildParentSurfaceHandoffRow(
  options: Infer<typeof AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffOptionsSchema>,
  row: ResponseConsumerHandoffRow,
  index: number
): Infer<typeof AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffRowSchema> {
  const parentSurfaceHandoffState = mapParentSurfaceHandoffState(
    row.parentSurfaceReadModelServiceReadApiResponseConsumerHandoffState
  );
  return AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${options.parentSurfaceHandoffId}-row-${index + 1}`,
    sourceResponseConsumerHandoffRowId: row.rowId,
    targetDomain: row.targetDomain,
    parentSurfaceHandoffState,
    inheritedServiceReadApiResponseConsumerProofRefs: row.requiredServiceReadApiResponseConsumerProofRefs,
    requiredParentSurfaceProofRefs:
      parentSurfaceHandoffState ===
      AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired
        ? options.parentSurfaceProofRefs
        : [],
    sourceEvidenceRefs: row.sourceEvidenceRefs,
    ...AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function mapParentSurfaceHandoffState(
  state: ResponseConsumerHandoffRow['parentSurfaceReadModelServiceReadApiResponseConsumerHandoffState']
): AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffStateValue {
  if (
    appGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffMatchesResponseConsumerHandoff(
      state,
      AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired
    )
  ) {
    return AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired;
  }
  if (
    appGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffMatchesResponseConsumerHandoff(
      state,
      AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.BlockedBySourceFreshness
    )
  ) {
    return AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.BlockedBySourceFreshness;
  }
  return AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.BlockedByCompilerDecision;
}
