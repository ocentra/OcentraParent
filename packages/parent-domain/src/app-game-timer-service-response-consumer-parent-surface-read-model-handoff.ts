import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import {
  AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffIdSchema as AppGameTimerServiceResponseConsumerParentSurfaceHandoffIdSchema,
  AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffRowIdSchema as AppGameTimerServiceResponseConsumerParentSurfaceHandoffRowIdSchema,
  AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffSchema as AppGameTimerServiceResponseConsumerParentSurfaceHandoffSchema,
  AppGameTimerServiceReadApiResponseConsumerParentSurfaceProofRefSchema as AppGameTimerServiceResponseConsumerParentSurfaceProofRefSchema,
  type AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffRowSchema as ParentSurfaceHandoffRowSchema,
} from './app-game-timer-service-read-api-response-consumer-parent-surface-handoff';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffNoClaimFlags,
  AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState,
  RequiredAppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffNonClaims,
  appGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffCountsMatch,
  appGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffHasNoRuntimeClaims,
  appGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffMatchesParentSurfaceHandoff,
  type AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffStateValue,
} from './app-game-timer-service-response-consumer-parent-surface-read-model-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const ParentSurfaceReadModelText = Schema.String.pipe(Schema.minLength(1));

type ParentSurfaceHandoffRow = Infer<typeof ParentSurfaceHandoffRowSchema>;

export const AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffIdSchema = ParentSurfaceReadModelText.pipe(
  Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffId')
);
export const AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffRowIdSchema =
  ParentSurfaceReadModelText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffRowId')
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceReadModelProofRefSchema = ParentSurfaceReadModelText.pipe(
  Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceReadModelProofRef')
);
export const AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffContractRefSchema =
  ParentSurfaceReadModelText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffContractRef')
  );

const ParentSurfaceReadModelHandoffStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState))
);
const ParentSurfaceReadModelHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffNonClaims)
);

export const AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    serviceResponseConsumerParentSurfaceReadModelHandoffId:
      AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffContractRefSchema),
    parentSurfaceReadModelProofRefs: Schema.Array(
      AppGameTimerServiceResponseConsumerParentSurfaceReadModelProofRefSchema
    ),
  }).pipe(
    Schema.filter(
      (options) =>
        (options.sourceContractRefs.length > 0 && options.parentSurfaceReadModelProofRefs.length > 0) ||
        'Expected service response consumer parent-surface read-model options to cite source contracts and read-model proof refs'
    )
  )
);

const ParentSurfaceReadModelHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffRowIdSchema,
  sourceServiceResponseConsumerParentSurfaceHandoffRowId:
    AppGameTimerServiceResponseConsumerParentSurfaceHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  serviceResponseConsumerParentSurfaceReadModelHandoffState: ParentSurfaceReadModelHandoffStateSchema,
  inheritedParentSurfaceProofRefs: Schema.Array(AppGameTimerServiceResponseConsumerParentSurfaceProofRefSchema),
  requiredParentSurfaceReadModelProofRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceReadModelProofRefSchema
  ),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  ...Object.fromEntries(
    Object.keys(AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffNoClaimFlags).map((key) => [
      key,
      Schema.Literal(false),
    ])
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffRowSchema = withParser(
  ParentSurfaceReadModelHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceReadModelHandoffState !==
          AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired ||
        (row.inheritedParentSurfaceProofRefs.length > 0 && row.requiredParentSurfaceReadModelProofRefs.length > 0) ||
        'Expected parent-surface read-model rows to preserve parent-surface proof refs and cite read-model proof refs'
    )
  )
);

const ParentSurfaceReadModelHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  serviceResponseConsumerParentSurfaceReadModelHandoffId:
    AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffIdSchema,
  sourceServiceResponseConsumerParentSurfaceHandoffId: AppGameTimerServiceResponseConsumerParentSurfaceHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffContractRefSchema),
  rows: Schema.Array(AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  parentSurfaceReadModelProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  serviceResponseConsumerParentSurfaceReadModelHandoffNonClaims: Schema.Array(
    ParentSurfaceReadModelHandoffNonClaimSchema
  ),
  ...Object.fromEntries(
    Object.keys(AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffNoClaimFlags).map((key) => [
      key,
      Schema.Literal(false),
    ])
  ),
});

export const AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffSchema = withParser(
  ParentSurfaceReadModelHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        appGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffCountsMatch(handoff) ||
        'Expected service response consumer parent-surface read-model handoff counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (handoff) =>
        appGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffHasNoRuntimeClaims(handoff) ||
        'Expected service response consumer parent-surface read-model handoff to keep runtime, rendering, adapter, child, platform, and raw-source claims false'
    )
  )
);

export function buildAppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoff(
  options: Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffOptionsSchema>,
  parentSurfaceHandoff: Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceHandoffSchema>
): Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffSchema> {
  const parsedOptions = AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffOptionsSchema.parse(options);
  const parsedParentSurfaceHandoff =
    AppGameTimerServiceResponseConsumerParentSurfaceHandoffSchema.parse(parentSurfaceHandoff);
  const rows = parsedParentSurfaceHandoff.rows.map((row, index) =>
    buildParentSurfaceReadModelHandoffRow(parsedOptions, row, index)
  );

  return AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffSchema.parse({
    schemaVersion: parsedOptions.schemaVersion,
    serviceResponseConsumerParentSurfaceReadModelHandoffId:
      parsedOptions.serviceResponseConsumerParentSurfaceReadModelHandoffId,
    sourceServiceResponseConsumerParentSurfaceHandoffId: parsedParentSurfaceHandoff.parentSurfaceHandoffId,
    generatedAt: parsedOptions.generatedAt,
    sourceContractRefs: parsedOptions.sourceContractRefs,
    rows,
    nativeAppRowCount: rows.filter((row) => row.targetDomain === 'native-app').length,
    nativeGameRowCount: rows.filter((row) => row.targetDomain === 'native-game').length,
    parentSurfaceReadModelProofRequiredCount: rows.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceReadModelHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceReadModelHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceReadModelHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.BlockedByCompilerDecision
    ).length,
    serviceResponseConsumerParentSurfaceReadModelHandoffNonClaims:
      RequiredAppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffNonClaims,
    ...AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffNoClaimFlags,
  });
}

function buildParentSurfaceReadModelHandoffRow(
  options: Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffOptionsSchema>,
  row: ParentSurfaceHandoffRow,
  index: number
): Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffRowSchema> {
  const parentSurfaceReadModelHandoffState = mapParentSurfaceReadModelHandoffState(row.parentSurfaceHandoffState);
  return AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${options.serviceResponseConsumerParentSurfaceReadModelHandoffId}-row-${index + 1}`,
    sourceServiceResponseConsumerParentSurfaceHandoffRowId: row.rowId,
    targetDomain: row.targetDomain,
    serviceResponseConsumerParentSurfaceReadModelHandoffState: parentSurfaceReadModelHandoffState,
    inheritedParentSurfaceProofRefs: row.requiredParentSurfaceProofRefs,
    requiredParentSurfaceReadModelProofRefs:
      parentSurfaceReadModelHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired
        ? options.parentSurfaceReadModelProofRefs
        : [],
    sourceEvidenceRefs: row.sourceEvidenceRefs,
    ...AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function mapParentSurfaceReadModelHandoffState(
  state: ParentSurfaceHandoffRow['parentSurfaceHandoffState']
): AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffStateValue {
  if (
    appGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffMatchesParentSurfaceHandoff(
      state,
      AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired
    )
  ) {
    return AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired;
  }
  if (
    appGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffMatchesParentSurfaceHandoff(
      state,
      AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.BlockedBySourceFreshness
    )
  ) {
    return AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.BlockedBySourceFreshness;
  }
  return AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.BlockedByCompilerDecision;
}
