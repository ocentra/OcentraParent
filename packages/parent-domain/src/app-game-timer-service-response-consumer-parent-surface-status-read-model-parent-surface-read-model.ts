import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffIdSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffRowIdSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelProofRefSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRefSchema,
  type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffRowSchema,
} from './app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNoClaimFlags,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState,
  RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNonClaims,
  appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelCountsMatch,
  appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHasNoRuntimeClaims,
  appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelMatchesHandoff,
  type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelStateValue,
} from './app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const ParentSurfaceText = Schema.String.pipe(Schema.minLength(1));

type HandoffRow = Infer<
  typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffRowSchema
>;

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelIdSchema =
  ParentSurfaceText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelId')
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowIdSchema =
  ParentSurfaceText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowId')
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelContractRefSchema =
  ParentSurfaceText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelContractRef')
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSummarySchema =
  ParentSurfaceText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSummary')
  );

const ParentSurfaceReadModelStateSchema = withParser(
  Schema.Literal(
    ...Object.values(AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState)
  )
);
const ParentSurfaceReadModelNonClaimSchema = withParser(
  Schema.Literal(
    ...RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNonClaims
  )
);

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelOptionsSchema =
  withParser(
    Schema.Struct({
      schemaVersion: ParentContractSchemaVersionSchema,
      parentSurfaceReadModelId:
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelIdSchema,
      generatedAt: ParentTimestampSchema,
      sourceContractRefs: Schema.Array(
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelContractRefSchema
      ),
    }).pipe(
      Schema.filter(
        (options) =>
          options.sourceContractRefs.length > 0 || 'Expected parent-surface read-model options to cite source contracts'
      )
    )
  );

const ParentSurfaceReadModelRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowIdSchema,
  sourceParentSurfaceReadModelHandoffRowId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  parentSurfaceReadModelState: ParentSurfaceReadModelStateSchema,
  parentSafeSummary: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSummarySchema,
  requiredParentSurfaceReadModelProofRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelProofRefSchema
  ),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  parentSurfaceReadModelRef:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRefSchema,
  ...Object.fromEntries(
    Object.keys(AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNoClaimFlags).map(
      (key) => [key, Schema.Literal(false)]
    )
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowSchema =
  withParser(
    ParentSurfaceReadModelRowBaseSchema.pipe(
      Schema.filter(
        (row) =>
          row.parentSurfaceReadModelState !==
            AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.ReadyForParentSurfaceReadModel ||
          row.requiredParentSurfaceReadModelProofRefs.length > 0 ||
          'Expected ready parent-surface read-model rows to cite required proof refs'
      )
    )
  );

const ParentSurfaceReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  parentSurfaceReadModelId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelIdSchema,
  sourceParentSurfaceReadModelHandoffId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelContractRefSchema
  ),
  rows: Schema.Array(AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  readyForParentSurfaceReadModelCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  parentSurfaceReadModelNonClaims: Schema.Array(ParentSurfaceReadModelNonClaimSchema),
  ...Object.fromEntries(
    Object.keys(AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNoClaimFlags).map(
      (key) => [key, Schema.Literal(false)]
    )
  ),
});

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema = withParser(
  ParentSurfaceReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelCountsMatch(readModel) ||
        'Expected parent-surface read-model counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (readModel) =>
        appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHasNoRuntimeClaims(
          readModel
        ) || 'Expected parent-surface read-model contract to keep runtime and rendering claims false'
    )
  )
);

export function buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModel(
  options: Infer<
    typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelOptionsSchema
  >,
  handoff: Infer<
    typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema
  >
): Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema> {
  const parsedOptions =
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelOptionsSchema.parse(options);
  const parsedHandoff =
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.parse(handoff);
  const rows = parsedHandoff.rows.map((row, index) => buildReadModelRow(parsedOptions, row, index));

  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema.parse({
    schemaVersion: parsedOptions.schemaVersion,
    parentSurfaceReadModelId: parsedOptions.parentSurfaceReadModelId,
    sourceParentSurfaceReadModelHandoffId:
      parsedHandoff.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffId,
    generatedAt: parsedOptions.generatedAt,
    sourceContractRefs: parsedOptions.sourceContractRefs,
    rows,
    nativeAppRowCount: rows.filter((row) => row.targetDomain === 'native-app').length,
    nativeGameRowCount: rows.filter((row) => row.targetDomain === 'native-game').length,
    readyForParentSurfaceReadModelCount: rows.filter(
      (row) =>
        row.parentSurfaceReadModelState ===
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.ReadyForParentSurfaceReadModel
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.parentSurfaceReadModelState ===
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.parentSurfaceReadModelState ===
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedByCompilerDecision
    ).length,
    parentSurfaceReadModelNonClaims:
      RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNonClaims,
    ...AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNoClaimFlags,
  });
}

function buildReadModelRow(
  options: Infer<
    typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelOptionsSchema
  >,
  row: HandoffRow,
  index: number
): Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowSchema> {
  const parentSurfaceReadModelState = mapReadModelState(
    row.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState
  );
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${options.parentSurfaceReadModelId}-row-${index + 1}`,
    sourceParentSurfaceReadModelHandoffRowId: row.rowId,
    targetDomain: row.targetDomain,
    parentSurfaceReadModelState,
    parentSafeSummary: `${row.targetDomain}:${parentSurfaceReadModelState}`,
    requiredParentSurfaceReadModelProofRefs: row.requiredParentSurfaceReadModelProofRefs,
    sourceEvidenceRefs: row.sourceEvidenceRefs,
    parentSurfaceReadModelRef: row.parentSurfaceReadModelRef,
    ...AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function mapReadModelState(
  state: HandoffRow['serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState']
): AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelStateValue {
  if (
    appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelMatchesHandoff(
      state,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.ReadyForParentSurfaceReadModel
    )
  ) {
    return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.ReadyForParentSurfaceReadModel;
  }
  if (
    appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelMatchesHandoff(
      state,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedBySourceFreshness
    )
  ) {
    return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedBySourceFreshness;
  }
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedByCompilerDecision;
}
