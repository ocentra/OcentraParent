import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerProofRefSchema,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffRowSchema as ServiceReadApiResponseConsumerHandoffRowSchema,
} from './app-game-timer-service-read-api-response-consumer-handoff';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceHandoffNoClaimFlags,
  AppGameTimerServiceResponseConsumerParentSurfaceHandoffState,
  RequiredAppGameTimerServiceResponseConsumerParentSurfaceHandoffNonClaims,
  appGameTimerServiceResponseConsumerParentSurfaceHandoffCountsMatch,
  appGameTimerServiceResponseConsumerParentSurfaceHandoffHasNoRuntimeClaims,
  appGameTimerServiceResponseConsumerParentSurfaceHandoffMatchesResponseConsumerHandoff,
  type AppGameTimerServiceResponseConsumerParentSurfaceHandoffStateValue,
} from './app-game-timer-service-response-consumer-parent-surface-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const ParentSurfaceHandoffText = Schema.String.pipe(Schema.minLength(1));

type ServiceReadApiResponseConsumerHandoffRow = Infer<typeof ServiceReadApiResponseConsumerHandoffRowSchema>;

export const AppGameTimerServiceResponseConsumerParentSurfaceHandoffIdSchema = ParentSurfaceHandoffText.pipe(
  Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceHandoffId')
);
export const AppGameTimerServiceResponseConsumerParentSurfaceHandoffRowIdSchema = ParentSurfaceHandoffText.pipe(
  Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceHandoffRowId')
);
export const AppGameTimerServiceResponseConsumerParentSurfaceProofRefSchema = ParentSurfaceHandoffText.pipe(
  Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceProofRef')
);
export const AppGameTimerServiceResponseConsumerParentSurfaceHandoffContractRefSchema = ParentSurfaceHandoffText.pipe(
  Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceHandoffContractRef')
);

const ParentSurfaceHandoffStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameTimerServiceResponseConsumerParentSurfaceHandoffState))
);
const ParentSurfaceHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameTimerServiceResponseConsumerParentSurfaceHandoffNonClaims)
);

export const AppGameTimerServiceResponseConsumerParentSurfaceHandoffOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    serviceResponseConsumerParentSurfaceHandoffId: AppGameTimerServiceResponseConsumerParentSurfaceHandoffIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(AppGameTimerServiceResponseConsumerParentSurfaceHandoffContractRefSchema),
    parentSurfaceProofRefs: Schema.Array(AppGameTimerServiceResponseConsumerParentSurfaceProofRefSchema),
  }).pipe(
    Schema.filter(
      (options) =>
        (options.sourceContractRefs.length > 0 && options.parentSurfaceProofRefs.length > 0) ||
        'Expected service response consumer parent-surface handoff options to cite source contracts and parent-surface proof refs'
    )
  )
);

const ParentSurfaceHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameTimerServiceResponseConsumerParentSurfaceHandoffRowIdSchema,
  sourceServiceReadApiResponseConsumerHandoffRowId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  serviceResponseConsumerParentSurfaceHandoffState: ParentSurfaceHandoffStateSchema,
  inheritedServiceReadApiResponseConsumerProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerProofRefSchema
  ),
  requiredParentSurfaceProofRefs: Schema.Array(AppGameTimerServiceResponseConsumerParentSurfaceProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  ...Object.fromEntries(
    Object.keys(AppGameTimerServiceResponseConsumerParentSurfaceHandoffNoClaimFlags).map((key) => [
      key,
      Schema.Literal(false),
    ])
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameTimerServiceResponseConsumerParentSurfaceHandoffRowSchema = withParser(
  ParentSurfaceHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceHandoffState !==
          AppGameTimerServiceResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired ||
        (row.inheritedServiceReadApiResponseConsumerProofRefs.length > 0 &&
          row.requiredParentSurfaceProofRefs.length > 0) ||
        'Expected parent-surface proof rows to preserve response consumer proof refs and cite parent-surface proof refs'
    )
  )
);

const ParentSurfaceHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  serviceResponseConsumerParentSurfaceHandoffId: AppGameTimerServiceResponseConsumerParentSurfaceHandoffIdSchema,
  sourceServiceReadApiResponseConsumerHandoffId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(AppGameTimerServiceResponseConsumerParentSurfaceHandoffContractRefSchema),
  rows: Schema.Array(AppGameTimerServiceResponseConsumerParentSurfaceHandoffRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  parentSurfaceProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  serviceResponseConsumerParentSurfaceHandoffNonClaims: Schema.Array(ParentSurfaceHandoffNonClaimSchema),
  ...Object.fromEntries(
    Object.keys(AppGameTimerServiceResponseConsumerParentSurfaceHandoffNoClaimFlags).map((key) => [
      key,
      Schema.Literal(false),
    ])
  ),
});

export const AppGameTimerServiceResponseConsumerParentSurfaceHandoffSchema = withParser(
  ParentSurfaceHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        appGameTimerServiceResponseConsumerParentSurfaceHandoffCountsMatch(handoff) ||
        'Expected service response consumer parent-surface handoff counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (handoff) =>
        appGameTimerServiceResponseConsumerParentSurfaceHandoffHasNoRuntimeClaims(handoff) ||
        'Expected service response consumer parent-surface handoff to keep runtime, rendering, adapter, child, platform, and raw-source claims false'
    )
  )
);

export function buildAppGameTimerServiceResponseConsumerParentSurfaceHandoff(
  options: Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceHandoffOptionsSchema>,
  serviceReadApiResponseConsumerHandoff: Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffSchema
  >
): Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceHandoffSchema> {
  const parsedOptions = AppGameTimerServiceResponseConsumerParentSurfaceHandoffOptionsSchema.parse(options);
  const parsedServiceReadApiResponseConsumerHandoff =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffSchema.parse(
      serviceReadApiResponseConsumerHandoff
    );
  const rows = parsedServiceReadApiResponseConsumerHandoff.rows.map((row, index) =>
    buildParentSurfaceHandoffRow(parsedOptions, row, index)
  );

  return AppGameTimerServiceResponseConsumerParentSurfaceHandoffSchema.parse({
    schemaVersion: parsedOptions.schemaVersion,
    serviceResponseConsumerParentSurfaceHandoffId: parsedOptions.serviceResponseConsumerParentSurfaceHandoffId,
    sourceServiceReadApiResponseConsumerHandoffId:
      parsedServiceReadApiResponseConsumerHandoff.parentSurfaceReadModelServiceReadApiResponseConsumerHandoffId,
    generatedAt: parsedOptions.generatedAt,
    sourceContractRefs: parsedOptions.sourceContractRefs,
    rows,
    nativeAppRowCount: rows.filter((row) => row.targetDomain === 'native-app').length,
    nativeGameRowCount: rows.filter((row) => row.targetDomain === 'native-game').length,
    parentSurfaceProofRequiredCount: rows.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceHandoffState.BlockedByCompilerDecision
    ).length,
    serviceResponseConsumerParentSurfaceHandoffNonClaims:
      RequiredAppGameTimerServiceResponseConsumerParentSurfaceHandoffNonClaims,
    ...AppGameTimerServiceResponseConsumerParentSurfaceHandoffNoClaimFlags,
  });
}

function buildParentSurfaceHandoffRow(
  options: Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceHandoffOptionsSchema>,
  row: ServiceReadApiResponseConsumerHandoffRow,
  index: number
): Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceHandoffRowSchema> {
  const parentSurfaceHandoffState = mapParentSurfaceHandoffState(
    row.parentSurfaceReadModelServiceReadApiResponseConsumerHandoffState
  );
  return AppGameTimerServiceResponseConsumerParentSurfaceHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${options.serviceResponseConsumerParentSurfaceHandoffId}-row-${index + 1}`,
    sourceServiceReadApiResponseConsumerHandoffRowId: row.rowId,
    targetDomain: row.targetDomain,
    serviceResponseConsumerParentSurfaceHandoffState: parentSurfaceHandoffState,
    inheritedServiceReadApiResponseConsumerProofRefs: row.requiredServiceReadApiResponseConsumerProofRefs,
    requiredParentSurfaceProofRefs:
      parentSurfaceHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired
        ? options.parentSurfaceProofRefs
        : [],
    sourceEvidenceRefs: row.sourceEvidenceRefs,
    ...AppGameTimerServiceResponseConsumerParentSurfaceHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function mapParentSurfaceHandoffState(
  state: ServiceReadApiResponseConsumerHandoffRow['parentSurfaceReadModelServiceReadApiResponseConsumerHandoffState']
): AppGameTimerServiceResponseConsumerParentSurfaceHandoffStateValue {
  if (
    appGameTimerServiceResponseConsumerParentSurfaceHandoffMatchesResponseConsumerHandoff(
      state,
      AppGameTimerServiceResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired
    )
  ) {
    return AppGameTimerServiceResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired;
  }
  if (
    appGameTimerServiceResponseConsumerParentSurfaceHandoffMatchesResponseConsumerHandoff(
      state,
      AppGameTimerServiceResponseConsumerParentSurfaceHandoffState.BlockedBySourceFreshness
    )
  ) {
    return AppGameTimerServiceResponseConsumerParentSurfaceHandoffState.BlockedBySourceFreshness;
  }
  return AppGameTimerServiceResponseConsumerParentSurfaceHandoffState.BlockedByCompilerDecision;
}
