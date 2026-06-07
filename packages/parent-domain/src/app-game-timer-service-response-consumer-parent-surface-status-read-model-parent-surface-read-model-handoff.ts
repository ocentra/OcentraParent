import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffIdSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffRowIdSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceProofRefSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceRefSchema,
  type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffRowSchema,
} from './app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-handoff';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffNoClaimFlags,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState,
  RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffNonClaims,
  appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffCountsMatch,
  appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffHasNoRuntimeClaims,
  appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffMatchesParentSurfaceHandoff,
  type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffStateValue,
} from './app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const ParentSurfaceText = Schema.String.pipe(Schema.minLength(1));

type ParentSurfaceHandoffRow = Infer<
  typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffRowSchema
>;

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffIdSchema =
  ParentSurfaceText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffId')
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffRowIdSchema =
  ParentSurfaceText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffRowId')
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelProofRefSchema =
  ParentSurfaceText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelProofRef')
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRefSchema =
  ParentSurfaceText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRef')
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelContractRefSchema =
  ParentSurfaceText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelContractRef')
  );

const ParentSurfaceReadModelHandoffStateSchema = withParser(
  Schema.Literal(
    ...Object.values(AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState)
  )
);
const ParentSurfaceReadModelHandoffNonClaimSchema = withParser(
  Schema.Literal(
    ...RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffNonClaims
  )
);

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffOptionsSchema =
  withParser(
    Schema.Struct({
      schemaVersion: ParentContractSchemaVersionSchema,
      serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffId:
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffIdSchema,
      generatedAt: ParentTimestampSchema,
      sourceContractRefs: Schema.Array(
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelContractRefSchema
      ),
      parentSurfaceReadModelProofRefs: Schema.Array(
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelProofRefSchema
      ),
      parentSurfaceReadModelRef:
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRefSchema,
    }).pipe(
      Schema.filter(
        (options) =>
          (options.sourceContractRefs.length > 0 && options.parentSurfaceReadModelProofRefs.length > 0) ||
          'Expected parent-surface read-model handoff options to cite source contracts and read-model proof refs'
      )
    )
  );

const ParentSurfaceReadModelHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffRowIdSchema,
  sourceServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffRowId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState:
    ParentSurfaceReadModelHandoffStateSchema,
  inheritedParentSurfaceProofRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceProofRefSchema
  ),
  requiredParentSurfaceReadModelProofRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelProofRefSchema
  ),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  parentSurfaceRef: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceRefSchema,
  parentSurfaceReadModelRef:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRefSchema,
  ...Object.fromEntries(
    Object.keys(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffRowSchema =
  withParser(
    ParentSurfaceReadModelHandoffRowBaseSchema.pipe(
      Schema.filter(
        (row) =>
          row.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState !==
            AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired ||
          (row.inheritedParentSurfaceProofRefs.length > 0 && row.requiredParentSurfaceReadModelProofRefs.length > 0) ||
          'Expected parent-surface read-model rows to preserve parent-surface proof refs and cite read-model proof refs'
      )
    )
  );

const ParentSurfaceReadModelHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffIdSchema,
  sourceServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelContractRefSchema
  ),
  parentSurfaceReadModelRef:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRefSchema,
  rows: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffRowSchema
  ),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  parentSurfaceReadModelProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffNonClaims: Schema.Array(
    ParentSurfaceReadModelHandoffNonClaimSchema
  ),
  ...Object.fromEntries(
    Object.keys(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
});

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema =
  withParser(
    ParentSurfaceReadModelHandoffBaseSchema.pipe(
      Schema.filter(
        (handoff) =>
          appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffCountsMatch(
            handoff
          ) || 'Expected parent-surface read-model handoff counts to match row states'
      )
    ).pipe(
      Schema.filter(
        (handoff) =>
          appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffHasNoRuntimeClaims(
            handoff
          ) ||
          'Expected parent-surface read-model handoff to keep runtime, persistence, rendering, adapter, child, platform, and raw-source claims false'
      )
    )
  );

export function buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoff(
  options: Infer<
    typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffOptionsSchema
  >,
  parentSurfaceHandoff: Infer<
    typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema
  >
): Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema> {
  const parsedOptions =
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffOptionsSchema.parse(
      options
    );
  const parsedParentSurfaceHandoff =
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema.parse(
      parentSurfaceHandoff
    );
  const rows = parsedParentSurfaceHandoff.rows.map((row, index) =>
    buildParentSurfaceReadModelHandoffRow(parsedOptions, row, index)
  );

  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.parse({
    schemaVersion: parsedOptions.schemaVersion,
    serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffId:
      parsedOptions.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffId,
    sourceServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffId:
      parsedParentSurfaceHandoff.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffId,
    generatedAt: parsedOptions.generatedAt,
    sourceContractRefs: parsedOptions.sourceContractRefs,
    parentSurfaceReadModelRef: parsedOptions.parentSurfaceReadModelRef,
    rows,
    nativeAppRowCount: rows.filter((row) => row.targetDomain === 'native-app').length,
    nativeGameRowCount: rows.filter((row) => row.targetDomain === 'native-game').length,
    parentSurfaceReadModelProofRequiredCount: rows.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.BlockedByCompilerDecision
    ).length,
    serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffNonClaims:
      RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffNonClaims,
    ...AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffNoClaimFlags,
  });
}

function buildParentSurfaceReadModelHandoffRow(
  options: Infer<
    typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffOptionsSchema
  >,
  row: ParentSurfaceHandoffRow,
  index: number
): Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffRowSchema> {
  const parentSurfaceReadModelHandoffState = mapParentSurfaceReadModelHandoffState(
    row.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState
  );
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${options.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffId}-row-${index + 1}`,
    sourceServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffRowId: row.rowId,
    targetDomain: row.targetDomain,
    serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState:
      parentSurfaceReadModelHandoffState,
    inheritedParentSurfaceProofRefs: row.requiredParentSurfaceProofRefs,
    requiredParentSurfaceReadModelProofRefs:
      parentSurfaceReadModelHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired
        ? options.parentSurfaceReadModelProofRefs
        : [],
    sourceEvidenceRefs: row.sourceEvidenceRefs,
    parentSurfaceRef: row.parentSurfaceRef,
    parentSurfaceReadModelRef: options.parentSurfaceReadModelRef,
    ...AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function mapParentSurfaceReadModelHandoffState(
  state: ParentSurfaceHandoffRow['serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState']
): AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffStateValue {
  if (
    appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffMatchesParentSurfaceHandoff(
      state,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired
    )
  ) {
    return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired;
  }
  if (
    appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffMatchesParentSurfaceHandoff(
      state,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.BlockedBySourceFreshness
    )
  ) {
    return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.BlockedBySourceFreshness;
  }
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.BlockedByCompilerDecision;
}
