import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffIdSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffRowIdSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceReadModelProofRefSchema,
  type AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffRowSchema as ParentSurfaceReadModelHandoffRowSchema,
} from './app-game-timer-service-response-consumer-parent-surface-read-model-handoff';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffNoClaimFlags,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState,
  RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffNonClaims,
  appGameTimerServiceResponseConsumerParentSurfaceStatusHandoffCountsMatch,
  appGameTimerServiceResponseConsumerParentSurfaceStatusHandoffHasNoRuntimeClaims,
  appGameTimerServiceResponseConsumerParentSurfaceStatusHandoffMatchesReadModelHandoff,
  type AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffStateValue,
} from './app-game-timer-service-response-consumer-parent-surface-status-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const ParentSurfaceStatusText = Schema.String.pipe(Schema.minLength(1));

type ParentSurfaceReadModelHandoffRow = Infer<typeof ParentSurfaceReadModelHandoffRowSchema>;

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffIdSchema = ParentSurfaceStatusText.pipe(
  Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffId')
);
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffRowIdSchema = ParentSurfaceStatusText.pipe(
  Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffRowId')
);
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusProofRefSchema = ParentSurfaceStatusText.pipe(
  Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusProofRef')
);
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffContractRefSchema =
  ParentSurfaceStatusText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffContractRef')
  );

const ParentSurfaceStatusHandoffStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState))
);
const ParentSurfaceStatusHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffNonClaims)
);

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    serviceResponseConsumerParentSurfaceStatusHandoffId:
      AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffContractRefSchema),
    parentSurfaceStatusProofRefs: Schema.Array(AppGameTimerServiceResponseConsumerParentSurfaceStatusProofRefSchema),
  }).pipe(
    Schema.filter(
      (options) =>
        (options.sourceContractRefs.length > 0 && options.parentSurfaceStatusProofRefs.length > 0) ||
        'Expected service response consumer parent-surface status options to cite source contracts and status proof refs'
    )
  )
);

const ParentSurfaceStatusHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffRowIdSchema,
  sourceServiceResponseConsumerParentSurfaceReadModelHandoffRowId:
    AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  serviceResponseConsumerParentSurfaceStatusHandoffState: ParentSurfaceStatusHandoffStateSchema,
  inheritedParentSurfaceReadModelProofRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceReadModelProofRefSchema
  ),
  requiredParentSurfaceStatusProofRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusProofRefSchema
  ),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  ...Object.fromEntries(
    Object.keys(AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffNoClaimFlags).map((key) => [
      key,
      Schema.Literal(false),
    ])
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffRowSchema = withParser(
  ParentSurfaceStatusHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceStatusHandoffState !==
          AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.ParentSurfaceStatusProofRequired ||
        (row.inheritedParentSurfaceReadModelProofRefs.length > 0 &&
          row.requiredParentSurfaceStatusProofRefs.length > 0) ||
        'Expected parent-surface status rows to preserve parent-surface read-model proof refs and cite status proof refs'
    )
  )
);

const ParentSurfaceStatusHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  serviceResponseConsumerParentSurfaceStatusHandoffId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffIdSchema,
  sourceServiceResponseConsumerParentSurfaceReadModelHandoffId:
    AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffContractRefSchema),
  rows: Schema.Array(AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  parentSurfaceStatusProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  serviceResponseConsumerParentSurfaceStatusHandoffNonClaims: Schema.Array(ParentSurfaceStatusHandoffNonClaimSchema),
  ...Object.fromEntries(
    Object.keys(AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffNoClaimFlags).map((key) => [
      key,
      Schema.Literal(false),
    ])
  ),
});

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffSchema = withParser(
  ParentSurfaceStatusHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        appGameTimerServiceResponseConsumerParentSurfaceStatusHandoffCountsMatch(handoff) ||
        'Expected service response consumer parent-surface status handoff counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (handoff) =>
        appGameTimerServiceResponseConsumerParentSurfaceStatusHandoffHasNoRuntimeClaims(handoff) ||
        'Expected service response consumer parent-surface status handoff to keep runtime, persistence, rendering, adapter, child, platform, and raw-source claims false'
    )
  )
);

export function buildAppGameTimerServiceResponseConsumerParentSurfaceStatusHandoff(
  options: Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffOptionsSchema>,
  parentSurfaceReadModelHandoff: Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffSchema>
): Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffSchema> {
  const parsedOptions = AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffOptionsSchema.parse(options);
  const parsedParentSurfaceReadModelHandoff =
    AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffSchema.parse(parentSurfaceReadModelHandoff);
  const rows = parsedParentSurfaceReadModelHandoff.rows.map((row, index) =>
    buildParentSurfaceStatusHandoffRow(parsedOptions, row, index)
  );

  return AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffSchema.parse({
    schemaVersion: parsedOptions.schemaVersion,
    serviceResponseConsumerParentSurfaceStatusHandoffId:
      parsedOptions.serviceResponseConsumerParentSurfaceStatusHandoffId,
    sourceServiceResponseConsumerParentSurfaceReadModelHandoffId:
      parsedParentSurfaceReadModelHandoff.serviceResponseConsumerParentSurfaceReadModelHandoffId,
    generatedAt: parsedOptions.generatedAt,
    sourceContractRefs: parsedOptions.sourceContractRefs,
    rows,
    nativeAppRowCount: rows.filter((row) => row.targetDomain === 'native-app').length,
    nativeGameRowCount: rows.filter((row) => row.targetDomain === 'native-game').length,
    parentSurfaceStatusProofRequiredCount: rows.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceStatusHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.ParentSurfaceStatusProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceStatusHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceStatusHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.BlockedByCompilerDecision
    ).length,
    serviceResponseConsumerParentSurfaceStatusHandoffNonClaims:
      RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffNonClaims,
    ...AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffNoClaimFlags,
  });
}

function buildParentSurfaceStatusHandoffRow(
  options: Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffOptionsSchema>,
  row: ParentSurfaceReadModelHandoffRow,
  index: number
): Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffRowSchema> {
  const parentSurfaceStatusHandoffState = mapParentSurfaceStatusHandoffState(
    row.serviceResponseConsumerParentSurfaceReadModelHandoffState
  );
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${options.serviceResponseConsumerParentSurfaceStatusHandoffId}-row-${index + 1}`,
    sourceServiceResponseConsumerParentSurfaceReadModelHandoffRowId: row.rowId,
    targetDomain: row.targetDomain,
    serviceResponseConsumerParentSurfaceStatusHandoffState: parentSurfaceStatusHandoffState,
    inheritedParentSurfaceReadModelProofRefs: row.requiredParentSurfaceReadModelProofRefs,
    requiredParentSurfaceStatusProofRefs:
      parentSurfaceStatusHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.ParentSurfaceStatusProofRequired
        ? options.parentSurfaceStatusProofRefs
        : [],
    sourceEvidenceRefs: row.sourceEvidenceRefs,
    ...AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function mapParentSurfaceStatusHandoffState(
  state: ParentSurfaceReadModelHandoffRow['serviceResponseConsumerParentSurfaceReadModelHandoffState']
): AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffStateValue {
  if (
    appGameTimerServiceResponseConsumerParentSurfaceStatusHandoffMatchesReadModelHandoff(
      state,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.ParentSurfaceStatusProofRequired
    )
  ) {
    return AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.ParentSurfaceStatusProofRequired;
  }
  if (
    appGameTimerServiceResponseConsumerParentSurfaceStatusHandoffMatchesReadModelHandoff(
      state,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.BlockedBySourceFreshness
    )
  ) {
    return AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.BlockedBySourceFreshness;
  }
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.BlockedByCompilerDecision;
}
