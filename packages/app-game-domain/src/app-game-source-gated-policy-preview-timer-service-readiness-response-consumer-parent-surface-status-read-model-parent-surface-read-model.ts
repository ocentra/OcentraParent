import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceProofRefSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceRefSchema,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffRowSchema,
} from './app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNonClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelCountsMatch,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelMatchesHandoff,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelStateValue,
} from './app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

type HandoffRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffRowSchema
>;

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelIdSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelId'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowIdSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowId'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelContractRefSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelContractRef'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSummarySchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSummary'
    )
  );

const ParentSurfaceReadModelStateSchema = withParser(
  Schema.Literal(
    ...Object.values(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState
    )
  )
);
const ParentSurfaceReadModelNonClaimSchema = withParser(
  Schema.Literal(
    ...RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNonClaims
  )
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelOptionsSchema =
  withParser(
    Schema.Struct({
      schemaVersion: ParentContractSchemaVersionSchema,
      parentSurfaceReadModelId:
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelIdSchema,
      generatedAt: ParentTimestampSchema,
      sourceContractRefs: Schema.Array(
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelContractRefSchema
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
  rowId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowIdSchema,
  sourceParentSurfaceReadModelHandoffRowId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  parentSurfaceReadModelState: ParentSurfaceReadModelStateSchema,
  parentSafeSummary:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSummarySchema,
  inheritedParentSurfaceStatusReadModelProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceProofRefSchema
  ),
  requiredParentSurfaceReadModelProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceProofRefSchema
  ),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  parentSurfaceStatusReadModelRef:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceRefSchema,
  parentSurfaceReadModelRef:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceRefSchema,
  ...Object.fromEntries(
    Object.keys(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowSchema =
  withParser(
    ParentSurfaceReadModelRowBaseSchema.pipe(
      Schema.filter(
        (row) =>
          row.parentSurfaceReadModelState !==
            AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.ReadyForParentSurfaceReadModel ||
          row.requiredParentSurfaceReadModelProofRefs.length > 0 ||
          'Expected ready parent-surface read-model rows to cite required proof refs'
      )
    )
  );

const ParentSurfaceReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  parentSurfaceReadModelId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelIdSchema,
  sourceParentSurfaceReadModelHandoffId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelContractRefSchema
  ),
  rows: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowSchema
  ),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  readyForParentSurfaceReadModelCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  parentSurfaceReadModelNonClaims: Schema.Array(ParentSurfaceReadModelNonClaimSchema),
  ...Object.fromEntries(
    Object.keys(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema =
  withParser(
    ParentSurfaceReadModelBaseSchema.pipe(
      Schema.filter(
        (readModel) =>
          appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelCountsMatch(
            readModel
          ) || 'Expected parent-surface read-model counts to match row states'
      )
    ).pipe(
      Schema.filter(
        (readModel) =>
          appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHasNoRuntimeClaims(
            readModel
          ) || 'Expected parent-surface read-model contract to keep runtime and rendering claims false'
      )
    )
  );

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModel =
  Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema
  >;

export function buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModel(
  options: Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelOptionsSchema
  >,
  handoff: Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema
  >
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModel {
  const parsedOptions =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelOptionsSchema.parse(
      options
    );
  const parsedHandoff =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.parse(
      handoff
    );
  const rows = parsedHandoff.rows.map((row, index) => buildReadModelRow(parsedOptions, row, index));
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema.parse(
    {
      schemaVersion: parsedOptions.schemaVersion,
      parentSurfaceReadModelId: parsedOptions.parentSurfaceReadModelId,
      sourceParentSurfaceReadModelHandoffId:
        parsedHandoff.responseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffId,
      generatedAt: parsedOptions.generatedAt,
      sourceContractRefs: parsedOptions.sourceContractRefs,
      rows,
      nativeAppRowCount: rows.filter((row) => row.targetDomain === 'native-app').length,
      nativeGameRowCount: rows.filter((row) => row.targetDomain === 'native-game').length,
      readyForParentSurfaceReadModelCount: rows.filter(
        (row) =>
          row.parentSurfaceReadModelState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.ReadyForParentSurfaceReadModel
      ).length,
      blockedBySourceFreshnessCount: rows.filter(
        (row) =>
          row.parentSurfaceReadModelState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedBySourceFreshness
      ).length,
      blockedByCompilerDecisionCount: rows.filter(
        (row) =>
          row.parentSurfaceReadModelState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedByCompilerDecision
      ).length,
      parentSurfaceReadModelNonClaims:
        RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNonClaims,
      ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNoClaimFlags,
    }
  );
}

function buildReadModelRow(
  options: Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelOptionsSchema
  >,
  row: HandoffRow,
  index: number
): Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowSchema
> {
  const parentSurfaceReadModelState = mapReadModelState(
    row.responseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState
  );
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowSchema.parse(
    {
      schemaVersion: options.schemaVersion,
      rowId: `${options.parentSurfaceReadModelId}-row-${index + 1}`,
      sourceParentSurfaceReadModelHandoffRowId: row.rowId,
      targetDomain: row.targetDomain,
      parentSurfaceReadModelState,
      parentSafeSummary: `${row.targetDomain}:${parentSurfaceReadModelState}`,
      inheritedParentSurfaceStatusReadModelProofRefs: row.inheritedParentSurfaceStatusReadModelProofRefs,
      requiredParentSurfaceReadModelProofRefs: row.requiredParentSurfaceReadModelProofRefs,
      sourceEvidenceRefs: row.sourceEvidenceRefs,
      parentSurfaceStatusReadModelRef: row.parentSurfaceStatusReadModelRef,
      parentSurfaceReadModelRef: row.parentSurfaceReadModelRef,
      ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNoClaimFlags,
      generatedAt: options.generatedAt,
    }
  );
}

function mapReadModelState(
  state: HandoffRow['responseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState']
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelStateValue {
  if (
    appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelMatchesHandoff(
      state,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.ReadyForParentSurfaceReadModel
    )
  ) {
    return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.ReadyForParentSurfaceReadModel;
  }
  if (
    appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelMatchesHandoff(
      state,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedBySourceFreshness
    )
  ) {
    return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedBySourceFreshness;
  }
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedByCompilerDecision;
}

