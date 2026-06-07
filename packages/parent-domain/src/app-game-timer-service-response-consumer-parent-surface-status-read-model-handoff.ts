import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffIdSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffRowIdSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusProofRefSchema,
  type AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffRowSchema as ParentSurfaceStatusHandoffRowSchema,
} from './app-game-timer-service-response-consumer-parent-surface-status-handoff';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffNoClaimFlags,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState,
  RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffNonClaims,
  appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffCountsMatch,
  appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffHasNoRuntimeClaims,
  appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffMatchesStatusHandoff,
  type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffStateValue,
} from './app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const ParentSurfaceStatusReadModelText = Schema.String.pipe(Schema.minLength(1));

type ParentSurfaceStatusHandoffRow = Infer<typeof ParentSurfaceStatusHandoffRowSchema>;

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffIdSchema =
  ParentSurfaceStatusReadModelText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffId')
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffRowIdSchema =
  ParentSurfaceStatusReadModelText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffRowId')
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelProofRefSchema =
  ParentSurfaceStatusReadModelText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelProofRef')
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffContractRefSchema =
  ParentSurfaceStatusReadModelText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffContractRef')
  );

const ParentSurfaceStatusReadModelHandoffStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState))
);
const ParentSurfaceStatusReadModelHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffNonClaims)
);

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    serviceResponseConsumerParentSurfaceStatusReadModelHandoffId:
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffContractRefSchema
    ),
    parentSurfaceStatusReadModelProofRefs: Schema.Array(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelProofRefSchema
    ),
  }).pipe(
    Schema.filter(
      (options) =>
        (options.sourceContractRefs.length > 0 && options.parentSurfaceStatusReadModelProofRefs.length > 0) ||
        'Expected service response consumer parent-surface status read-model options to cite source contracts and status read-model proof refs'
    )
  )
);

const ParentSurfaceStatusReadModelHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffRowIdSchema,
  sourceServiceResponseConsumerParentSurfaceStatusHandoffRowId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  serviceResponseConsumerParentSurfaceStatusReadModelHandoffState: ParentSurfaceStatusReadModelHandoffStateSchema,
  inheritedParentSurfaceStatusProofRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusProofRefSchema
  ),
  requiredParentSurfaceStatusReadModelProofRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelProofRefSchema
  ),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  ...Object.fromEntries(
    Object.keys(AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffNoClaimFlags).map((key) => [
      key,
      Schema.Literal(false),
    ])
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffRowSchema = withParser(
  ParentSurfaceStatusReadModelHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceStatusReadModelHandoffState !==
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.ParentSurfaceStatusReadModelProofRequired ||
        (row.inheritedParentSurfaceStatusProofRefs.length > 0 &&
          row.requiredParentSurfaceStatusReadModelProofRefs.length > 0) ||
        'Expected parent-surface status read-model rows to preserve parent-surface status proof refs and cite status read-model proof refs'
    )
  )
);

const ParentSurfaceStatusReadModelHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  serviceResponseConsumerParentSurfaceStatusReadModelHandoffId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffIdSchema,
  sourceServiceResponseConsumerParentSurfaceStatusHandoffId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffContractRefSchema
  ),
  rows: Schema.Array(AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  parentSurfaceStatusReadModelProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  serviceResponseConsumerParentSurfaceStatusReadModelHandoffNonClaims: Schema.Array(
    ParentSurfaceStatusReadModelHandoffNonClaimSchema
  ),
  ...Object.fromEntries(
    Object.keys(AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffNoClaimFlags).map((key) => [
      key,
      Schema.Literal(false),
    ])
  ),
});

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffSchema = withParser(
  ParentSurfaceStatusReadModelHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffCountsMatch(handoff) ||
        'Expected service response consumer parent-surface status read-model handoff counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (handoff) =>
        appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffHasNoRuntimeClaims(handoff) ||
        'Expected service response consumer parent-surface status read-model handoff to keep runtime, persistence, rendering, adapter, child, platform, and raw-source claims false'
    )
  )
);

export function buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoff(
  options: Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffOptionsSchema>,
  parentSurfaceStatusHandoff: Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffSchema>
): Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffSchema> {
  const parsedOptions =
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffOptionsSchema.parse(options);
  const parsedParentSurfaceStatusHandoff =
    AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffSchema.parse(parentSurfaceStatusHandoff);
  const rows = parsedParentSurfaceStatusHandoff.rows.map((row, index) =>
    buildParentSurfaceStatusReadModelHandoffRow(parsedOptions, row, index)
  );

  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffSchema.parse({
    schemaVersion: parsedOptions.schemaVersion,
    serviceResponseConsumerParentSurfaceStatusReadModelHandoffId:
      parsedOptions.serviceResponseConsumerParentSurfaceStatusReadModelHandoffId,
    sourceServiceResponseConsumerParentSurfaceStatusHandoffId:
      parsedParentSurfaceStatusHandoff.serviceResponseConsumerParentSurfaceStatusHandoffId,
    generatedAt: parsedOptions.generatedAt,
    sourceContractRefs: parsedOptions.sourceContractRefs,
    rows,
    nativeAppRowCount: rows.filter((row) => row.targetDomain === 'native-app').length,
    nativeGameRowCount: rows.filter((row) => row.targetDomain === 'native-game').length,
    parentSurfaceStatusReadModelProofRequiredCount: rows.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceStatusReadModelHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.ParentSurfaceStatusReadModelProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceStatusReadModelHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceStatusReadModelHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.BlockedByCompilerDecision
    ).length,
    serviceResponseConsumerParentSurfaceStatusReadModelHandoffNonClaims:
      RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffNonClaims,
    ...AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffNoClaimFlags,
  });
}

function buildParentSurfaceStatusReadModelHandoffRow(
  options: Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffOptionsSchema>,
  row: ParentSurfaceStatusHandoffRow,
  index: number
): Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffRowSchema> {
  const parentSurfaceStatusReadModelHandoffState = mapParentSurfaceStatusReadModelHandoffState(
    row.serviceResponseConsumerParentSurfaceStatusHandoffState
  );
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${options.serviceResponseConsumerParentSurfaceStatusReadModelHandoffId}-row-${index + 1}`,
    sourceServiceResponseConsumerParentSurfaceStatusHandoffRowId: row.rowId,
    targetDomain: row.targetDomain,
    serviceResponseConsumerParentSurfaceStatusReadModelHandoffState: parentSurfaceStatusReadModelHandoffState,
    inheritedParentSurfaceStatusProofRefs: row.requiredParentSurfaceStatusProofRefs,
    requiredParentSurfaceStatusReadModelProofRefs:
      parentSurfaceStatusReadModelHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.ParentSurfaceStatusReadModelProofRequired
        ? options.parentSurfaceStatusReadModelProofRefs
        : [],
    sourceEvidenceRefs: row.sourceEvidenceRefs,
    ...AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function mapParentSurfaceStatusReadModelHandoffState(
  state: ParentSurfaceStatusHandoffRow['serviceResponseConsumerParentSurfaceStatusHandoffState']
): AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffStateValue {
  if (
    appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffMatchesStatusHandoff(
      state,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.ParentSurfaceStatusReadModelProofRequired
    )
  ) {
    return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.ParentSurfaceStatusReadModelProofRequired;
  }
  if (
    appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffMatchesStatusHandoff(
      state,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.BlockedBySourceFreshness
    )
  ) {
    return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.BlockedBySourceFreshness;
  }
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.BlockedByCompilerDecision;
}
